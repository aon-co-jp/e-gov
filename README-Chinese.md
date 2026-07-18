# e-gov.info(简体中文)

一个整合项目：(1) 数字政府(无纸化行政手续)与 (2) 面向个人到贸易公司的
在线贸易/不动产平台，通过LINE应用、网站、以及便利店多媒体终端等多种入口
提供服务。

> ⚠️ **本项目目前仅为样品/演示，并非正式服务。**
> 无论通过网站还是添加LINE官方账号好友，每次都会显示本提示。电子公证与
> 具有法律约束力的电子合同(不动产买卖/租赁)目前尚未实现，需等待正式批准。

## 两大支柱

### 1. 数字政府

- **多种入口**：除LINE应用与网站外，还为不熟悉网站/智能手机操作的用户
  提供便利店多媒体终端(Loppi、Fami Port、7-Eleven复合打印机等)作为
  申请窗口。
- **分级身份验证**：根据交易金额/重要程度调整验证强度(来电显示+回拨确认
  → 邮件一次性密码 → My Number卡的手机NFC扫描)。
- 以无纸化在线申请、跨多个政府机构/地方自治体的一站式服务为目标。

### 2. 在线贸易平台(AI聊天商务)

- 从食品到汽车、音响设备，通过与AI的对话在LINE与网站上均可发现并
  下单的综合电商。
- 面向个人到贸易公司的市场平台模式，手续费大幅低于现有大型平台。
- **AI信用调查、赊购与应收账款担保**：根据AI计算的信用评分实现延期
  付款采购、电子发票重复检测、应收账款担保功能。
- **不动产投资与AI建筑事务所**：AI根据搜索到的土地信息提出户型方案，
  设计原则是不助长投机资金过度流入不动产市场。
- 初期阶段不进行实际库存采购，以广告收入方式作为AI聊天商务用户体验
  的示范运营。

## 参考的海外案例

- 爱沙尼亚 e-Estonia / X-Road(去中心化数据交换基础设施)
- 阿塞拜疆 ASAN xidmet(一站式服务中心、流动服务)

## 自动调查与自动营销功能

针对数字政府相关主题，以日语和英语定期进行自动调查(通过GitHub Search API
获取真实数据、自动生成Google搜索链接)，并定期自动生成各功能的宣传文案
(参见`/research`页面)。

## LINE联动

除浏览网站外，您还可以添加LINE官方账号为好友，通过聊天进行咨询(基于
关键词的规则型AI聊天商务回复，例如"申请"、"购买"、"赊购"、"寻找土地"等)。
添加好友用的二维码将在LINE官方账号的Basic ID发放后提供。

## 来自GitHub的自动展示

首页会从GitHub实时获取本仓库的`README.md`、`CLAUDE.md`、`PORTING.md`，
并以GitHub风格渲染显示(可切换为`.rs`格式显示)。

## 技术栈

Rust + [Poem](https://github.com/poem-web/poem)。不依赖数据库，单一
可执行文件。与`aruaru-tokyo`/`karu.tokyo`采用相同技术栈。

详细设计理念请参阅[CLAUDE.md](CLAUDE.md)。

## 相关项目

- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 开发规则正本
- [aruaru-db](https://github.com/aon-co-jp/aruaru-db) — 未来数据库连接候选
- [aruaru-tokyo](https://github.com/aon-co-jp/aruaru-tokyo-server) / [karu.tokyo](https://github.com/aon-co-jp/karu.tokyo) — 采用相同技术栈的姊妹网站
- [aruaru-llm](https://github.com/aon-co-jp/aruaru-llm) / [open-cuda](https://github.com/aon-co-jp/open-cuda) — 无需签约的自研AI组合，聊天商务的未来后端候选
