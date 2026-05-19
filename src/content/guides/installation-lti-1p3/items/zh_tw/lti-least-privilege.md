FastComments LTI 1.3 整合遵循最小權限原則：它僅使用識別使用者、將評論附加到正確課程與資源，以及套用基於角色的權限所需的啟動宣告（launch claims）。

本頁其餘部分對照表列出整合所使用的每個宣告、沒有請求的每項 LTI Advantage 服務，以及不會收集的每類資料。資安與採購審查人員可以直接從下表擷取答案。

## 從 LMS 接收的資料元素

每次 LTI 1.3 啟動都會攜帶 LMS 簽署的 JWT。FastComments 從該 JWT 擷取下列宣告，且不使用其他資料：

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | 在多次啟動中一致地識別使用者，確保同一個人會對應到相同的 FastComments SSO 使用者 | 是 | 是，作為穩定的內部 SSO ID 的一部分 |
| Display name | `name` | 顯示於使用者評論旁的署名 | 是（若缺少則退回為 "LMS User"） | 是 |
| Email | `email` | 帳戶比對、通知、審核、支援對應 | 選填（整合在沒有電子郵件的情況下仍可運作） | 若提供則會儲存 |
| Avatar URL | `picture` | 顯示在使用者的評論上 | 選填 | 僅儲存 URL；FastComments 不會下載或重新托管圖片 |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | 決定使用者是管理員、講師（審核者）或學習者 | 是 | 在 SSO session 上為衍生的 `isAdmin` / `isModerator` 標記 |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | 將評論串連結到正確的 LMS 課程 | 是 | 是，作為解析後頁面識別器的一部分 |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | 將評論與課程內正確的活動或工具位置關聯 | 若存在則為必要 | 是，作為解析後頁面識別器的一部分 |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | 將啟動路由到正確的 FastComments 租戶設定 | 是 | 是，儲存在 FastComments 的 LTI 設定紀錄上 |

## 註冊時宣告的宣告與範圍

在 LTI 1.3 動態註冊期間，FastComments 以 `scope: ""`（無額外 OAuth 範圍）註冊，並僅宣告下列 OpenID Connect 宣告：

`iss`, `sub`, `name`, `email`, `picture`

它註冊了兩種訊息類型：

- `LtiResourceLinkRequest` - 標準的課程啟動以進入 FastComments。
- `LtiDeepLinkingRequest` - 允許講師將 FastComments 工具放置到課程中。

不會向 LMS 請求其他的存取權杖。

## 未請求的 LTI Advantage 服務

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | 否 | 整合不需要課程名冊；使用者身份會隨每次啟動一併傳來 |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | 否 | 此整合不會與成績簿整合 |
| Deep Linking beyond the standard placement return | 無額外資料 | Deep linking 僅用於講師放置工具；不會列舉課程內容 |

## 未收集的資料

除了 LTI 本身之外，FastComments 不會向 LMS 或使用者請求或接收下列資料：

| Category | Collected? |
|----------|------------|
| Student grades | 否 |
| Assignment submissions | 否 |
| Attendance records | 否 |
| Full course rosters | 否 |
| Government identifiers | 否 |
| Date of birth | 否 |
| Postal address or phone number | 否 |
| Financial information | 否 |
| LMS administrator credentials | 否 |

## 存取邊界

- FastComments 僅接收由 LMS 註冊金鑰簽署的授權 LTI 1.3 啟動中的資料。整合不會回呼 LMS 以取得額外資訊。
- 啟動權杖為一次性且短時效。重放或過期的權杖會被拒絕。
- LMS 管理員控制工具在其平台內的部署位置。例如 D2L Brightspace 支援每一部署的 org-unit 範圍設定與每一部署的安全性設定，允許管理員將工具限制在特定課程或組織單位，而不是在全域中開放。Moodle、Blackboard、Sakai 與 Schoology 在其 LTI 1.3 實作中提供相當的每部署控制。

## 儲存與保留

FastComments 會在評論服務有效期間以及根據客戶所設定的保留政策保留 LTI 衍生資料。評論資料儲存在靜態加密（encrypted-at-rest）的生產儲存中。於帳戶終止或收到書面刪除請求時，FastComments 會依適用協議刪除或匿名化客戶資料。

完整的儲存與資料處理細節，請參閱 <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments 隱私政策</a>。

## 審查頻率

任何需要額外宣告、範圍或 LTI Advantage 服務的新 LTI 功能，在發布前都會進行審查，以確認所請求的存取權與將提供的功能之間是必要且相稱的。

## 作答資安問卷的簡短說明

> FastComments 在其 LTI 1.3 整合中採用最小權限與資料最小化原則。整合僅使用驗證使用者所需的 LTI 啟動宣告（`sub`, `name`, `email`, `picture`）、判定其角色，以及識別評論所屬課程與資源。FastComments 不會請求 Names and Role Provisioning Services、Assignment and Grade Services、成績簿資料、出席資料、完整名冊或 LMS 管理存取權。LMS 管理員保有控制工具可用於哪些組織單位、課程與部署的權限。