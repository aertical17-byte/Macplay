import AppKit
import SwiftUI
import UniformTypeIdentifiers

@main
struct MacPlayApp: App {
    var body: some Scene {
        WindowGroup {
            LauncherView().frame(minWidth: 760, minHeight: 500)
        }
        .windowStyle(.hiddenTitleBar)
    }
}

private struct GameEntry: Identifiable, Codable {
    let id: UUID
    let title: String
    let path: String
    let status: String

    init(url: URL) {
        id = UUID()
        title = url.deletingPathExtension().lastPathComponent
        path = url.path
        status = "Importato · non testato"
    }
}

private enum GameLibrary {
    private static var fileURL: URL {
        FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0]
            .appendingPathComponent("MacPlay", isDirectory: true)
            .appendingPathComponent("library.json")
    }

    static func load() -> [GameEntry] {
        guard let data = try? Data(contentsOf: fileURL),
              let games = try? JSONDecoder().decode([GameEntry].self, from: data) else { return [] }
        return games
    }

    static func save(_ games: [GameEntry]) throws {
        let directory = fileURL.deletingLastPathComponent()
        try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        try encoder.encode(games).write(to: fileURL, options: .atomic)
    }
}

private struct LauncherView: View {
    @State private var games = GameLibrary.load()
    @State private var errorMessage: String?

    var body: some View {
        HStack(spacing: 0) {
            VStack(alignment: .leading, spacing: 22) {
                Label("MacPlay", systemImage: "gamecontroller.fill")
                    .font(.system(size: 23, weight: .bold))
                Divider()
                Label("Libreria", systemImage: "square.grid.2x2.fill").font(.headline)
                Spacer()
                Text("ANTEPRIMA SVILUPPO").font(.caption2.weight(.bold)).foregroundStyle(.secondary)
            }
            .padding(22)
            .frame(width: 210)
            .background(.quaternary.opacity(0.35))

            VStack(alignment: .leading, spacing: 20) {
                HStack {
                    VStack(alignment: .leading, spacing: 5) {
                        Text("La tua libreria").font(.largeTitle.bold())
                        Text("I tuoi giochi Windows, raccolti in un unico posto.")
                            .foregroundStyle(.secondary)
                    }
                    Spacer()
                    Button(action: importExecutable) {
                        Label("Importa .exe", systemImage: "plus")
                    }
                    .buttonStyle(.borderedProminent)
                }
                Divider()
                if games.isEmpty {
                    ContentUnavailableView(
                        "Nessun gioco aggiunto",
                        systemImage: "gamecontroller",
                        description: Text("Importa un eseguibile Windows. Per ora MacPlay salva il riferimento al file: non lo esegue.")
                    )
                } else {
                    List {
                        ForEach(games) { game in
                            HStack(spacing: 12) {
                                Image(systemName: "gamecontroller")
                                    .font(.title2)
                                    .frame(width: 44, height: 44)
                                    .background(.quaternary, in: RoundedRectangle(cornerRadius: 10))
                                VStack(alignment: .leading, spacing: 4) {
                                    Text(game.title).font(.headline)
                                    Text(game.path).font(.caption).foregroundStyle(.secondary)
                                        .lineLimit(1).truncationMode(.middle)
                                }
                                Spacer()
                                Text(game.status).font(.caption).foregroundStyle(.secondary)
                            }
                            .padding(.vertical, 5)
                            .contextMenu {
                                Button("Rimuovi dalla libreria", role: .destructive) { remove(game) }
                            }
                        }
                    }
                }
                Spacer()
                Text("Runtime Windows e avvio dei giochi sono ancora in sviluppo. DRM e anti-cheat possono impedire l'esecuzione.")
                    .font(.caption).foregroundStyle(.secondary)
            }
            .padding(28)
        }
        .background(.background)
        .alert("Operazione non riuscita", isPresented: Binding(
            get: { errorMessage != nil },
            set: { if !$0 { errorMessage = nil } }
        )) {
            Button("OK", role: .cancel) { errorMessage = nil }
        } message: {
            Text(errorMessage ?? "Errore sconosciuto")
        }
    }

    private func importExecutable() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [UTType(filenameExtension: "exe") ?? .item]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        guard panel.runModal() == .OK, let url = panel.url else { return }
        guard !games.contains(where: { $0.path == url.path }) else {
            errorMessage = "Questo eseguibile è già presente nella libreria."
            return
        }
        persist(games + [GameEntry(url: url)])
    }

    private func remove(_ game: GameEntry) {
        persist(games.filter { $0.id != game.id })
    }

    private func persist(_ updated: [GameEntry]) {
        do {
            try GameLibrary.save(updated)
            games = updated
        } catch {
            errorMessage = "Impossibile salvare la libreria: \(error.localizedDescription)"
        }
    }
}
