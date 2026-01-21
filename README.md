# cmdr

A desktop application for managing development projects and terminal workspaces. Organize your projects, define reusable terminal layouts, and launch pre-configured multi-pane terminal sessions with a single click.

## Features

- **Project Management** - Add, edit, and organize development projects with descriptions and tags
- **Workspace Layouts** - Define terminal layouts with configurable pane grids (rows × columns)
- **Terminal Automation** - Launch workspaces with multiple panes, each with its own working directory, commands, and environment variables
- **Environment Configurations** - Create named environment variable sets per project
- **Quick Launch Dashboard** - Access recent workspaces and projects from the home screen
- **iTerm2 Integration** - Native AppleScript integration with iTerm2 on macOS

## Tech Stack

- **Frontend**: Vue 3, TypeScript, Vite, Pinia, PrimeVue
- **Backend**: Tauri 2, Rust
- **Storage**: TOML configuration files in `~/.config/cmdr/`

## Requirements

- Node.js 20+
- pnpm
- Rust toolchain
- macOS (currently the primary supported platform)
- iTerm2 (recommended)

## Development

```bash
# Install dependencies
pnpm install

# Start development mode (runs both frontend and Tauri)
pnpm tauri dev
```

The frontend dev server runs on `http://localhost:1420`. Tauri watches for changes and hot-reloads automatically.

## Building

```bash
# Build for production
pnpm tauri build
```

Build output is located in `src-tauri/target/release/bundle/`.

## Code Signing (macOS)

To distribute the app outside the Mac App Store, you need to code sign and notarize it. This requires an Apple Developer account.

### Certificate Setup

You need a **Developer ID Application** certificate (not "Apple Development" or "Apple Distribution"):

1. Go to [developer.apple.com/account/resources/certificates](https://developer.apple.com/account/resources/certificates)
2. Click **+** to create a new certificate
3. Select **Developer ID Application**
4. Follow the steps to create and download the certificate
5. Double-click the downloaded certificate to install it in Keychain Access

### Export the Certificate

1. Open **Keychain Access**
2. Find your **Developer ID Application** certificate under "My Certificates"
3. Right-click → Export → Save as a `.p12` file with a strong password
4. Base64 encode it for GitHub Actions:
   ```bash
   base64 -i certificate.p12 -o certificate-base64.txt
   ```

### Get Your Team ID

1. Go to [developer.apple.com/account](https://developer.apple.com/account)
2. Find your Team ID in the membership section (10-character string)

### Create an App-Specific Password

1. Go to [appleid.apple.com](https://appleid.apple.com)
2. Sign in and go to **Security** → **App-Specific Passwords**
3. Generate a new password for "cmdr notarization"

### GitHub Actions Secrets

Add these secrets to your repository (**Settings → Secrets and variables → Actions**):

| Secret | Description |
|--------|-------------|
| `APPLE_CERTIFICATE` | Contents of `certificate-base64.txt` |
| `APPLE_CERTIFICATE_PASSWORD` | Password used when exporting the .p12 |
| `APPLE_SIGNING_IDENTITY` | Full certificate name, e.g., `Developer ID Application: Your Company (ABC123XYZ)` |
| `APPLE_ID` | Your Apple ID email address |
| `APPLE_PASSWORD` | App-specific password from appleid.apple.com |
| `APPLE_TEAM_ID` | Your 10-character Team ID |

Once configured, the GitHub Actions workflow will automatically sign and notarize the app during builds.

### Why Base64 Encode the Certificate?

GitHub Secrets can only store text, not binary data. The .p12 certificate is a binary file, so it must be base64 encoded to store it as a secret. The workflow decodes it back to binary before importing it into the CI keychain.

### Local Signing

For local builds, if your Developer ID Application certificate is in your Keychain, Tauri will automatically sign the app. Set these environment variables for notarization:

```bash
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Company (ABC123XYZ)"
export APPLE_ID="your@email.com"
export APPLE_PASSWORD="your-app-specific-password"
export APPLE_TEAM_ID="ABC123XYZ"
pnpm tauri build
```

## License

MIT
