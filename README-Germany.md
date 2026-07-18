# e-gov.info (Deutsch)

Ein Projekt, das (1) digitales Regierungshandeln (papierlose
Verwaltungsverfahren) und (2) eine Online-Handels-/Immobilienplattform
für Privatpersonen bis hin zu Handelsgesellschaften vereint, zugänglich
über mehrere Zugangswege: eine LINE-App, eine Website und
Multimedia-Terminals in Convenience-Stores.

> ⚠️ **DIES IST NOCH LEDIGLICH EIN MUSTER / EINE DEMONSTRATION — KEIN
> LIVE-DIENST.** Dieser Hinweis wird jedes Mal angezeigt, egal ob Sie die
> Website nutzen oder das LINE-Konto als Freund hinzufügen. Elektronische
> Beglaubigung und rechtsverbindliche elektronische Verträge
> (Immobilienkauf/-miete) sind noch nicht umgesetzt und warten auf eine
> formelle behördliche Genehmigung.

## Zwei Säulen

### 1. Digitales Regierungshandeln

- **Mehrere Zugangswege**: eine LINE-App und eine Website sowie
  Multimedia-Terminals in Convenience-Stores (Loppi, Fami Port,
  7-Eleven-Multifunktionskopierer) für Menschen, die mit Websites oder
  Smartphones nicht vertraut sind.
- **Gestufte Identitätsprüfung**: die Prüfstärke skaliert mit dem
  Wert/der Bedeutung der Transaktion (Anrufer-ID + Rückruf → E-Mail-OTP
  → NFC-Scan der My-Number-Karte per Smartphone).
- Ziel sind papierlose Online-Anträge und eine One-Stop-Erfahrung über
  mehrere Behörden/Kommunen hinweg.

### 2. Online-Handelsplattform (KI-Chat-Commerce)

- Ein allgemeiner Marktplatz, von Lebensmitteln bis zu Autos und
  Audiogeräten, auffindbar und bestellbar per KI-Gespräch sowohl auf LINE
  als auch im Web.
- Ein Marktplatzmodell für Privatpersonen bis hin zu
  Handelsgesellschaften, mit deutlich niedrigeren Gebühren als bei
  bestehenden großen Plattformen.
- **KI-Bonitätsprüfung, Handelskredit und Forderungsgarantie**: Kauf auf
  Ziel auf Basis eines KI-berechneten Bonitätswerts, Erkennung doppelter
  Rechnungen und Garantien für Forderungen.
- **Immobilieninvestition und KI-Bauunternehmen**: eine KI schlägt
  Grundrisse auf Basis der recherchierten Grundstücksinformationen vor,
  mit dem Gestaltungsprinzip, keinen spekulativen Kapitalzufluss in
  Immobilien zu befördern.
- Die Anfangsphase läuft ohne echten Lagerbestand, monetarisiert durch
  Werbeeinnahmen, als Demonstration der KI-Chat-Commerce-Nutzererfahrung.

## Untersuchte Referenzmodelle

- Estlands e-Estonia / X-Road (dezentrale Datenaustausch-Infrastruktur)
- Aserbaidschans ASAN xidmet (One-Stop-Servicezentren, mobile
  Außendienste)

## Automatisierte Recherche und Marketing

Recherchiert regelmäßig digitale Regierungsthemen sowohl auf Japanisch
als auch auf Englisch (echte GitHub-Search-API-Aufrufe, generierte
Google-Suchlinks) und entwirft regelmäßig Marketingtexte für jede
Funktion (siehe die Seite `/research`).

## LINE-Integration

Neben dem Durchsuchen der Website können Sie das offizielle LINE-Konto
als Freund hinzufügen und im Chat Fragen stellen (eine regelbasierte
KI-Chat-Commerce-Antwort, ausgelöst durch Schlüsselwörter wie „Antrag“,
„Kauf“, „Handelskredit“, „Grundstückssuche“). Ein QR-Code zum Hinzufügen
als Freund wird veröffentlicht, sobald die Basic ID des offiziellen
LINE-Kontos ausgestellt ist.

## Automatische Anzeige von GitHub

Die Startseite ruft die Dateien `README.md`, `CLAUDE.md` und
`PORTING.md` dieses Repositorys live von GitHub ab und rendert sie im
GitHub-Stil (mit einer Umschaltmöglichkeit zur `.rs`-Stil-Darstellung).

## Technologie-Stack

Rust + [Poem](https://github.com/poem-web/poem). Keine
Datenbankabhängigkeit, eigenständige Einzelbinärdatei. Gleicher
Technologie-Stack wie `aruaru-tokyo`/`karu.tokyo`.

Siehe [CLAUDE.md](CLAUDE.md) für die vollständige Designphilosophie.

## Verwandte Projekte

- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — kanonische Quelle der Entwicklungsrichtlinien
- [aruaru-db](https://github.com/aon-co-jp/aruaru-db) — künftiger Kandidat für die Datenbankanbindung
- [aruaru-tokyo](https://github.com/aon-co-jp/aruaru-tokyo-server) / [karu.tokyo](https://github.com/aon-co-jp/karu.tokyo) — Schwesterseiten auf demselben Technologie-Stack
- [aruaru-llm](https://github.com/aon-co-jp/aruaru-llm) / [open-cuda](https://github.com/aon-co-jp/open-cuda) — vertragsfreies eigenes KI-SET, künftiger Backend-Kandidat für Chat-Commerce
