# Steam discovery scope

The Steam discovery crate parses local Valve KeyValues text files only:
- app manifests (AppState, app ID, display name, install directory);
- library folder metadata and app ID lists.

It does not read Steam credentials, cookies, login state or session files. It does not authenticate, modify Steam files, install games, or launch a title. Discovery metadata is not proof of Windows compatibility or even proof that a game executable is present. The future UI must clearly distinguish detected, installed, imported and tested states.
