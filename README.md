## Roadmap

# FreeFlowy Roadmap TODO

## Phase 1: Self-host UX parity and stability

- [x] Verify web and desktop both load workspace content without reconnect loops in a clean browser profile.
- [x] Run a full end-to-end smoke pass (signup/login/workspace open/edit/sync) on both clients and capture a repeatable checklist.
- [x] Remove remaining paywall and plan-gating UI for self-hosted usage.
  - [x] Hide workspace menu plan actions (`Change Plan`, `Get AI Max`) on self-hosted web.
  - [x] Hide guest-share upgrade modal path on self-hosted web (show neutral availability message instead).
  - [x] Hide version-history upgrade banner on self-hosted web.
  - [x] Replace AI paywall prompts with neutral availability messaging in desktop AI chat.
  - [x] Remove `Upgrade to Pro Plan` surfaces from self-host runtime paths.
  - [x] Remove `Get AI Max` surfaces from self-host runtime paths.

## Phase 2: Branding migration (AppFlowy -> FreeFlowy)

- [ ] Replace logo assets across desktop, web, and cloud-facing surfaces with FreeFlowy branding.
- [ ] Update app icons, favicon, splash/loading artwork, and in-app brand marks.
- [x] Replace visible product naming from `AppFlowy` to `FreeFlowy` in UI text.
- [ ] Audit code/config identifiers and user-facing strings that still reference `appflowy` and rename where appropriate.

### Phase 2 execution plan

- [x] Lock branding scope and rename policy (user-visible first; defer risky internal identifiers).
- [x] Build a cross-repo branding inventory with concrete file targets.
- [ ] Replace web branding surfaces (title/meta/logo/favicon/help/download links/translations).
  - [x] Update first-pass web user-visible text (title/meta/login/help/error/primary English translations).
  - [ ] Replace web logo/icon/image assets once FreeFlowy exports are ready.
    - [x] Replace core web logo assets (`public/appflowy.ico`, `public/appflowy.svg`, `src/assets/icons/appflowy.svg`, `src/assets/icons/logo.svg`).
    - [ ] Replace remaining web visual assets (`public/og-image.png`, `cypress/fixtures/appflowy.png`) if needed.
- [ ] Replace desktop branding surfaces (window/app name, platform metadata, splash/logo assets, key UI strings).
  - [x] Update first-pass desktop user-visible text (`error_page`, starter template copy).
  - [ ] Replace desktop logo/splash/icon assets once FreeFlowy exports are ready.
    - [x] Replace primary desktop/docs visuals (splash, `flowy_logo*.svg`, docs logo/title images).
    - [ ] Replace remaining desktop packaging/runtime icon assets (`assets/flowy_icons/40x/*`, linux packaging/flatpak branding files).
- [ ] Replace cloud branding surfaces (admin pages, mail templates, workspace starter templates, mailer strings).
  - [x] Update first-pass cloud admin UI and mailer subject text.
  - [ ] Update cloud email template body copy and workspace starter template copy.
  - [x] Replace cloud logo/image assets once FreeFlowy exports are ready.
- [ ] Run regression smoke on desktop + web + cloud after branding changes.

## Phase 3: Documentation alignment

- [ ] Update setup docs for local and VPS self-host deployment using current FreeFlowy flow.
- [ ] Update auth/billing behavior docs to reflect self-host-compatible defaults.
- [ ] Update screenshots/examples and naming to FreeFlowy.
- [ ] Add a "known limitations" section for deferred items (for example, guest sharing and AI service optionality).

## Phase 4: Release readiness

- [ ] Create a final QA matrix for desktop + web + cloud self-host scenario.
- [ ] Define migration/versioning notes for teams moving from older local setups.
- [ ] Prepare a tagged release checklist (build, artifacts, changelog, rollback notes).

## Built With

- [Flutter](https://flutter.dev/)

- [Rust](https://www.rust-lang.org/)

## Contributing

## License

Distributed under the AGPLv3 License. See [`LICENSE.md`](https://github.com/AppFlowy-IO/AppFlowy/blob/main/LICENSE) for
more information.
