# e-gov.info

A project unifying (1) digital government (paperless administrative
services) and (2) an online trade / real-estate platform for individuals
through trading companies, accessible through multiple entry points: a
LINE app, a website, and convenience-store multimedia terminals.

> ⚠️ **THIS IS STILL A SAMPLE / DEMONSTRATION ONLY — NOT A LIVE SERVICE.**
> This notice appears every time, whether you use the website or add the
> LINE account as a friend. Electronic notarization and legally binding
> electronic contracts (real-estate sale/lease) are not implemented yet,
> pending formal regulatory approval.

## Two pillars

### 1. Digital government

- **Multiple entry points**: a LINE app and a website, plus convenience-
  store multimedia terminals (Loppi, Fami Port, 7-Eleven multi-copy
  machines) for people unfamiliar with websites or smartphones.
- **Tiered identity verification**: verification strength scales with the
  value/importance of the transaction (caller-ID + callback → email OTP
  → My Number card NFC scan via smartphone).
- Aims for paperless online applications and a one-stop experience across
  multiple government agencies/municipalities.

### 2. Online trade platform (AI chat commerce)

- A general marketplace, from groceries to cars and audio equipment,
  discoverable and orderable through AI conversation on both LINE and the
  web.
- A marketplace model for individuals through trading companies, with
  significantly lower fees than existing major platforms.
- **AI credit scoring, trade credit, and receivables guarantee**:
  deferred-payment purchasing based on an AI-computed credit score,
  duplicate-invoice detection, and accounts-receivable guarantees.
- **Real-estate investment and AI home-builder**: an AI suggests floor
  plans based on searched land information, with a design principle of
  not fueling speculative capital inflow into real estate.
- The initial phase runs without real inventory, monetized via ad revenue
  as a demonstration of the AI chat-commerce UX.

## Reference models studied

- Estonia's e-Estonia / X-Road (decentralized data-exchange backbone)
- Azerbaijan's ASAN xidmet (one-stop service centers, mobile outreach)

## Automated research and marketing

Periodically researches digital-government topics in both Japanese and
English (real GitHub Search API calls, generated Google search links),
and periodically drafts marketing copy for each feature (see the
`/research` page).

## LINE integration

Besides browsing the website, you can add the LINE Official Account as a
friend and ask questions in chat (a rule-based AI chat-commerce reply,
triggered by keywords like "apply", "buy", "credit purchase", "land
search"). A QR code for adding the friend will be published once the LINE
Official Account's Basic ID is issued.

## Auto-displayed from GitHub

The TOP page fetches this repository's `README.md`, `CLAUDE.md`, and
`PORTING.md` live from GitHub and renders them GitHub-style (with a
toggle to a `.rs`-style rendering).

## Tech stack

Rust + [Poem](https://github.com/poem-web/poem). No DB dependency, single
self-contained binary. Same stack as `aruaru-tokyo`/`karu.tokyo`.

See [CLAUDE.md](CLAUDE.md) for the full design philosophy.

## Related projects

- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — canonical dev-policy source
- [aruaru-db](https://github.com/aon-co-jp/aruaru-db) — future DB connection candidate
- [aruaru-tokyo](https://github.com/aon-co-jp/aruaru-tokyo-server) / [karu.tokyo](https://github.com/aon-co-jp/karu.tokyo) — sister sites on the same tech stack
- [aruaru-llm](https://github.com/aon-co-jp/aruaru-llm) / [open-cuda](https://github.com/aon-co-jp/open-cuda) — contract-free in-house AI SET, intended future backend for chat commerce
