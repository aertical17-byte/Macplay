import SwiftUI

@main
struct MacPlayApp: App {
    var body: some Scene {
        WindowGroup {
            LauncherView()
                .frame(minWidth: 760, minHeight: 500)
        }
        .windowStyle(.hiddenTitleBar)
    }
}

private struct GameEntry: Identifiable {
    let id = UUID()
    let title: String
    let detail: String
    let status: String
}

private struct LauncherView: View {
    @State private var games: [GameEntry] = [
        GameEntry(title: "La tua libreria", detail: "Aggiungi un file .exe per iniziare", status: "Nessun gioco")
    ]
    @State private var selectedGame: UUID?

    var body: some View {
        HStack(spacing: 0) {
            VStack(alignment: .leading, spacing: 22) {
                Label("MacPlay", systemImage: "gamecontroller.fill")
                    .font(.system(size: 23, weight: .bold))
                Divider()
                Label("Libreria", systemImage: "square.grid.2x2.fill")
                    .font(.headline)
                Spacer()
                Text("ANTEPRIMA SVILUPPO")
                    .font(.caption2.weight(.bold))
                    .foregroundStyle(.secondary)
            }
            .padding(22)
            .frame(width: 210)
            .background(.quaternary.opacity(0.35))

            VStack(alignment: .leading, spacing: 20) {
                HStack {
                    VStack(alignment: .leading, spacing: 5) {
                        Text("La tua libreria").font(.largeTitle.bold())
                        Text("Un punto di partenza per i giochi Windows su macOS.")
                            .foregroundStyle(.secondary)
                    }
                    Spacer()
                    Button(action: importExecutable) {
                        Label("Importa .exe", systemImage: "plus")
                    }
                    .buttonStyle(.borderedProminent)
                }
                Divider()
                if games.count == 1 && games[0].title == "La tua libreria" {
                    ContentUnavailableView(
                        "Nessun gioco aggiunto",
                        systemImage: "gamecontroller",
                        description: Text("Importa un installer o un eseguibile Windows. In questa build viene registrato solo il percorso: non viene ancora eseguito.")
                    )
                } else {
                    List(games) { game in
                        HStack {
                            Image(systemName: "gamecontroller")
                                .font(.title2)
                                .frame(width: 44, height: 44)
                                .background(.quaternary, in: RoundedRectangle(cornerRadius: 10))
                            VStack(alignment: .leading) {
                                Text(game.title).font(.headline)
                                Text(game.detail).font(.caption).foregroundStyle(.secondary)
                            }
                            Spacer()
                            Text(game.status).font(.caption).foregroundStyle(.secondary)
                        }
                        .padding(.vertical, 5)
                    }
                }
                Spacer()
                Text("MacPlay non include ancora Wine né avvia giochi. La compatibilità dipende dal titolo, dal runtime, dai DRM e dagli anti-cheat.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            .padding(28)
        }
        .background(.background)
    }

    private func importExecutable() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.exe, .item]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        guard panel.runModal() == .OK, let url = panel.url else { return }
        games.append(GameEntry(
            title: url.deletingPathExtension().lastPathComponent,
            detail: url.path,
            status: "Importato · non testato"
        ))
    }
}

import AppKit
import UniformTypeIdentifiers

private extension UTType {
    static let exe = UTType(filenameExtension: "exe") ?? .item
}
