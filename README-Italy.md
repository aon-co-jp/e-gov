# e-gov.info (Italiano)

Un progetto che unifica (1) il governo digitale (pratiche amministrative
senza carta) e (2) una piattaforma di commercio online / immobiliare per
privati e società commerciali, accessibile tramite più canali: un'app
LINE, un sito web e terminali multimediali nei convenience store.

> ⚠️ **QUESTO È ANCORA SOLO UN CAMPIONE / DIMOSTRAZIONE — NON UN SERVIZIO
> ATTIVO.** Questo avviso appare ogni volta, sia che usiate il sito web
> sia che aggiungiate l'account LINE come amico. L'autenticazione
> notarile elettronica e i contratti elettronici vincolanti (vendita/
> locazione immobiliare) non sono ancora implementati, in attesa
> dell'approvazione normativa formale.

## Due pilastri

### 1. Governo digitale

- **Più punti di accesso**: un'app LINE e un sito web, oltre a terminali
  multimediali nei convenience store (Loppi, Fami Port, fotocopiatrici
  multifunzione 7-Eleven) per chi non ha familiarità con siti web o
  smartphone.
- **Verifica dell'identità a livelli**: l'intensità della verifica scala
  con il valore/l'importanza della transazione (ID chiamante + richiamata
  → OTP via email → scansione NFC della carta My Number tramite
  smartphone).
- Punta a domande online senza carta e a un'esperienza "one-stop" tra più
  enti pubblici/amministrazioni locali.

### 2. Piattaforma di commercio online (AI chat commerce)

- Un marketplace generale, dal cibo alle automobili e apparecchiature
  audio, ricercabile e ordinabile tramite conversazione con l'AI sia su
  LINE che sul web.
- Un modello di marketplace per privati fino alle società commerciali,
  con commissioni significativamente più basse rispetto alle grandi
  piattaforme esistenti.
- **Valutazione del credito AI, credito commerciale e garanzia sui
  crediti**: acquisti a pagamento differito basati su un punteggio di
  credito calcolato dall'AI, rilevamento di fatture duplicate e garanzie
  sui crediti.
- **Investimento immobiliare e AI home-builder**: un'AI suggerisce piante
  in base alle informazioni sul terreno ricercato, con il principio di
  progettazione di non alimentare afflussi speculativi di capitale nel
  mercato immobiliare.
- La fase iniziale opera senza inventario reale, monetizzata tramite
  pubblicità come dimostrazione dell'esperienza utente dell'AI chat
  commerce.

## Modelli di riferimento studiati

- e-Estonia / X-Road (infrastruttura decentralizzata di scambio dati)
- ASAN xidmet dell'Azerbaigian (centri servizi one-stop, unità mobili)

## Ricerca e marketing automatizzati

Ricerca periodicamente argomenti sul governo digitale sia in giapponese
che in inglese (chiamate reali alla GitHub Search API, link di ricerca
Google generati), e redige periodicamente bozze di marketing per ogni
funzionalità (vedi la pagina `/research`).

## Integrazione LINE

Oltre a navigare il sito web, potete aggiungere l'account ufficiale LINE
come amico e porre domande in chat (una risposta AI chat-commerce basata
su regole, attivata da parole chiave come "richiesta", "acquisto",
"credito commerciale", "ricerca terreno"). Un codice QR per aggiungere
l'amico sarà pubblicato una volta emesso il Basic ID dell'account
ufficiale LINE.

## Visualizzazione automatica da GitHub

La pagina principale recupera in tempo reale `README.md`, `CLAUDE.md` e
`PORTING.md` di questo repository da GitHub e li rende in stile GitHub
(con un'opzione per la visualizzazione in stile `.rs`).

## Stack tecnologico

Rust + [Poem](https://github.com/poem-web/poem). Nessuna dipendenza da
database, binario unico autonomo. Stesso stack tecnologico di
`aruaru-tokyo`/`karu.tokyo`.

Vedi [CLAUDE.md](CLAUDE.md) per la filosofia di design completa.

## Progetti correlati

- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — fonte canonica delle policy di sviluppo
- [aruaru-db](https://github.com/aon-co-jp/aruaru-db) — futuro candidato per la connessione al database
- [aruaru-tokyo](https://github.com/aon-co-jp/aruaru-tokyo-server) / [karu.tokyo](https://github.com/aon-co-jp/karu.tokyo) — siti gemelli sullo stesso stack tecnologico
- [aruaru-llm](https://github.com/aon-co-jp/aruaru-llm) / [open-cuda](https://github.com/aon-co-jp/open-cuda) — SET di AI proprietaria senza contratti esterni, futuro backend candidato per la chat commerce
