# e-gov.info(繁體中文)

一個整合專案：(1) 數位政府(無紙化行政手續)與 (2) 面向個人到貿易公司的
線上貿易/不動產平台，透過LINE應用程式、網站、以及便利商店多媒體終端等
多種入口提供服務。

> ⚠️ **本專案目前僅為樣品/展示，並非正式服務。**
> 無論透過網站或加LINE官方帳號好友，每次都會顯示本提示。電子公證與具有
> 法律約束力的電子合約(不動產買賣/租賃)目前尚未實作，需等待正式核准。

## 兩大支柱

### 1. 數位政府

- **多種入口**：除LINE應用程式與網站外，還為不熟悉網站/智慧型手機操作
  的使用者提供便利商店多媒體終端(Loppi、Fami Port、7-Eleven影印機等)
  作為申請窗口。
- **分級身分驗證**：依交易金額/重要程度調整驗證強度(來電顯示+回撥確認
  → 電子郵件一次性密碼 → My Number卡的手機NFC掃描)。
- 以無紙化線上申請、跨多個政府機關/地方自治體的一站式服務為目標。

### 2. 線上貿易平台(AI聊天商務)

- 從食品到汽車、音響設備，透過與AI對話在LINE與網站上皆可發現並下單的
  綜合電商。
- 面向個人到貿易公司的市場平台模式，手續費大幅低於現有大型平台。
- **AI信用調查、賒購與應收帳款保證**：依AI計算的信用評分實現延期付款
  採購、電子發票重複檢測、應收帳款保證功能。
- **不動產投資與AI建築事務所**：AI依搜尋到的土地資訊提出格局方案，設計
  原則是不助長投機資金過度流入不動產市場。
- 初期階段不進行實際庫存採購，以廣告收益方式作為AI聊天商務使用者體驗的
  示範營運。

## 參考的海外案例

- 愛沙尼亞 e-Estonia / X-Road(去中心化資料交換基礎設施)
- 亞塞拜然 ASAN xidmet(一站式服務中心、行動服務)

## 自動調查與自動行銷功能

針對數位政府相關主題，以日語和英語定期進行自動調查(透過GitHub Search API
取得真實資料、自動產生Google搜尋連結)，並定期自動產生各功能的宣傳文案
(參見`/research`頁面)。

## LINE串接

除瀏覽網站外，您還可以加LINE官方帳號好友，透過聊天進行諮詢(基於關鍵字的
規則型AI聊天商務回覆，例如「申請」、「購買」、「賒購」、「尋找土地」等)。
加好友用的QR Code將在LINE官方帳號的Basic ID發放後提供。

## 來自GitHub的自動展示

首頁會即時從GitHub取得本儲存庫的`README.md`、`CLAUDE.md`、`PORTING.md`，
並以GitHub風格渲染顯示(可切換為`.rs`格式顯示)。

## 技術堆疊

Rust + [Poem](https://github.com/poem-web/poem)。不依賴資料庫，單一
可執行檔。與`aruaru-tokyo`/`karu.tokyo`採用相同技術堆疊。

詳細設計理念請參閱[CLAUDE.md](CLAUDE.md)。

## 相關專案

- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開發規則正本
- [aruaru-db](https://github.com/aon-co-jp/aruaru-db) — 未來資料庫連接候選
- [aruaru-tokyo](https://github.com/aon-co-jp/aruaru-tokyo-server) / [karu.tokyo](https://github.com/aon-co-jp/karu.tokyo) — 採用相同技術堆疊的姊妹網站
- [aruaru-llm](https://github.com/aon-co-jp/aruaru-llm) / [open-cuda](https://github.com/aon-co-jp/open-cuda) — 免簽約的自研AI組合，聊天商務的未來後端候選
