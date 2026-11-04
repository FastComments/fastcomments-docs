當 FastComments 中啟用 SAML 時,系統會自動產生您需要在身份提供者中設定的服務提供者(SP)資訊。

### 存取服務提供者資訊

啟用 SAML 驗證後,SP 資訊會顯示在您的 SAML 設定頁面中。此資訊包含您的身份提供者建立 SAML 信任關係所需的所有詳細資料。

### 服務提供者端點

#### SP Entity ID / Audience
**用途**:唯一識別您的 FastComments 執行個體作為服務提供者
**格式**:`https://fastcomments.com/saml/{your-tenant-id}`
**用法**:在您的 IdP 中將此設定為 Entity ID 或 Audience

此識別碼確保 SAML 回應是針對您特定的 FastComments 租戶,並防止 SAML 回應被其他執行個體接受。

#### Assertion Consumer Service(ACS)URL
**用途**:您的 IdP 在使用者驗證後傳送 SAML 回應的端點
**格式**:`https://fastcomments.com/saml/callback/{your-tenant-id}`
**用法**:在您的 IdP 中將此設定為 ACS URL 或 Reply URL

這是使用者透過您的身份提供者成功驗證後被重新導向的位置,同時包含使用者資訊的 SAML 斷言。

#### SP Metadata URL
**用途**:以標準 XML 格式提供完整的 SAML 設定
**格式**:`https://fastcomments.com/saml/metadata/{your-tenant-id}`
**用法**:某些 IdP 可以使用此 URL 自動匯入設定

Metadata URL 以 XML 格式包含所有必要的 SP 資訊,讓相容的身份提供者可以輕鬆自動設定。

#### SAML 登入 URL
**用途**:為您的租戶啟動 SAML 驗證的直接連結
**格式**:`https://fastcomments.com/saml/login/{your-tenant-id}`
**用法**:將使用者直接連結至 SAML 驗證或測試流程

您可以使用此 URL 測試 SAML 驗證,或為使用者提供透過 SAML 登入的直接連結。

### SAML 綁定支援

FastComments 支援以下 SAML 綁定:

#### HTTP-POST 綁定
- **主要方法**:最常用的 SAML 回應綁定方式
- **安全性**:SAML 回應透過 HTTP POST 傳送至 ACS URL
- **用法**:建議用於正式環境部署

#### HTTP-Redirect 綁定
- **替代方法**:透過 HTTP 重新導向傳送 SAML 回應
- **限制**:由於 URL 長度限制,承載量有限
- **用法**:支援,但建議使用 HTTP-POST

### Name ID 政策

FastComments 在 SAML 請求中設定以下 Name ID 政策:

- **預設格式**:`urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **替代格式**:Persistent、Transient、Unspecified(可設定)
- **要求**:電子郵件地址作為主要使用者識別碼

### SAML 請求屬性

啟動 SAML 驗證時,FastComments 會傳送具有以下特性的請求:

#### 請求簽署
- **狀態**:選用(可設定)
- **演算法**:符合設定的簽署演算法
- **憑證**:若啟用請求簽署,則使用租戶專屬憑證

#### 請求的屬性
FastComments 在 SAML AuthnRequests 中請求以下屬性:

- **Email**:使用者識別所需
- **First Name**:選用,用於顯示目的
- **Last Name**:選用,用於顯示目的
- **Roles/Groups**:選用,用於存取控制與權限

### 複製 SP 資訊

SAML 設定頁面提供可點擊的欄位,可自動將 SP 資訊複製到您的剪貼簿:

1. 點擊任何 SP 資訊欄位(Entity ID、ACS URL 等)
2. 該值會自動複製到您的剪貼簿
3. 將該值貼到您的身份提供者設定中
4. 簡短的高亮顯示表示複製成功

這讓您可以輕鬆準確地將 SP 資訊轉移到您的 IdP,而不會產生輸入錯誤。

### SP 憑證資訊

#### 憑證用途
- **用途**:加密通訊並驗證 SP 身份
- **輪替**:憑證由 FastComments 自動管理
- **存取**:公開憑證可透過 metadata URL 取得

#### 憑證詳細資訊
- **演算法**:RSA-2048 或更高
- **有效期**:憑證會在到期前自動更新
- **發佈**:可透過標準 SAML metadata 取得

### SP 設定疑難排解

如果您的身份提供者回報 SP 資訊有問題:

1. **驗證 URL**:確保所有 URL 使用 HTTPS 並包含正確的租戶 ID
2. **檢查 Metadata**:使用 metadata URL 驗證設定
3. **測試連線**:確保您的 IdP 可以連線至 FastComments 端點
4. **驗證格式**:確認您的 IdP 支援 SP 資訊格式

常見問題包括:
- URL 中的租戶 ID 不正確
- IdP 與 FastComments 之間的網路連線問題
- IdP 預期不同的 URL 格式或需要額外的設定選項