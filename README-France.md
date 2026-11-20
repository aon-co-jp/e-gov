# e-gov.info (Français)

Un projet unifiant (1) le gouvernement numérique (démarches
administratives sans papier) et (2) une plateforme de commerce en ligne
/ immobilier pour les particuliers jusqu'aux sociétés commerciales,
accessible via plusieurs points d'entrée : une application LINE, un site
web et des terminaux multimédias dans les supérettes.

> ⚠️ **CECI N'EST ENCORE QU'UN ÉCHANTILLON / UNE DÉMONSTRATION — CE N'EST
> PAS UN SERVICE EN LIGNE ACTIF.** Cet avis s'affiche à chaque fois, que
> vous utilisiez le site web ou que vous ajoutiez le compte LINE comme
> ami. La notarisation électronique et les contrats électroniques
> juridiquement contraignants (vente/location immobilière) ne sont pas
> encore mis en œuvre, en attente d'une approbation réglementaire
> formelle.

## Deux piliers

### 1. Gouvernement numérique

- **Points d'entrée multiples** : une application LINE et un site web,
  ainsi que des terminaux multimédias en supérette (Loppi, Fami Port,
  photocopieurs multifonctions 7-Eleven) pour les personnes peu à l'aise
  avec les sites web ou les smartphones.
- **Vérification d'identité par paliers** : l'intensité de la
  vérification s'ajuste selon la valeur/l'importance de la transaction
  (identification de l'appelant + rappel → OTP par e-mail → lecture NFC
  de la carte My Number via smartphone).
- Vise des démarches en ligne sans papier et une expérience à guichet
  unique entre plusieurs administrations/collectivités locales.

### 2. Plateforme de commerce en ligne (commerce conversationnel par IA)

- Une place de marché généraliste, de l'alimentaire aux voitures et au
  matériel audio, consultable et commandable par conversation avec l'IA
  sur LINE comme sur le web.
- Un modèle de place de marché pour les particuliers jusqu'aux sociétés
  commerciales, avec des frais nettement inférieurs à ceux des grandes
  plateformes existantes.
- **Notation de crédit par IA, crédit commercial et garantie de
  créances** : achats à paiement différé fondés sur un score de crédit
  calculé par IA, détection de factures dupliquées et garanties de
  créances clients.
- **Investissement immobilier et IA constructeur** : une IA propose des
  plans d'aménagement à partir des informations sur le terrain
  recherché, avec le principe de conception de ne pas alimenter un afflux
  spéculatif de capitaux dans l'immobilier.
- La phase initiale fonctionne sans stock réel, monétisée par la
  publicité, à titre de démonstration de l'expérience utilisateur du
  commerce conversationnel par IA.

## Modèles de référence étudiés

- e-Estonia / X-Road de l'Estonie (infrastructure décentralisée
  d'échange de données)
- ASAN xidmet de l'Azerbaïdjan (centres de services à guichet unique,
  unités mobiles)

## Recherche et marketing automatisés

Effectue périodiquement des recherches sur les thèmes du gouvernement
numérique en japonais et en anglais (appels réels à l'API GitHub Search,
génération de liens de recherche Google), et rédige périodiquement des
brouillons marketing pour chaque fonctionnalité (voir la page
`/research`).

## Intégration LINE

Outre la navigation sur le site web, vous pouvez ajouter le compte
officiel LINE comme ami et poser des questions par chat (une réponse de
commerce conversationnel par IA basée sur des règles, déclenchée par des
mots-clés comme « demande », « achat », « crédit commercial », «
recherche de terrain »). Un code QR pour ajouter l'ami sera publié une
fois l'identifiant de base du compte officiel LINE émis.

## Affichage automatique depuis GitHub

La page d'accueil récupère en direct depuis GitHub les fichiers
`README.md`, `CLAUDE.md` et `PORTING.md` de ce dépôt et les affiche dans
le style GitHub (avec une bascule vers un affichage de type `.rs`).

## Pile technologique

Rust + [Poem](https://github.com/poem-web/poem). Aucune dépendance à une
base de données, binaire unique autonome. Même pile technologique que
`aruaru-tokyo`/`karu.tokyo`.

Voir [CLAUDE.md](CLAUDE.md) pour la philosophie de conception complète.

## Projets associés

- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — source canonique des politiques de développement
- [aruaru-db](https://github.com/aon-co-jp/aruaru-db) — futur candidat de connexion à une base de données
- [aruaru-tokyo](https://github.com/aon-co-jp/aruaru-tokyo-server) / [karu.tokyo](https://github.com/aon-co-jp/karu.tokyo) — sites jumeaux sur la même pile technologique
- [aruaru-llm](https://github.com/aon-co-jp/aruaru-llm) / [open-cuda](https://github.com/aon-co-jp/open-cuda) — SET d'IA maison sans contrat externe, futur backend candidat pour le commerce conversationnel
