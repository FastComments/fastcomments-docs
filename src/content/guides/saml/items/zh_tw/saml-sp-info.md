當在 FastComments 啟用 SAML 時，系統會自動產生您需要在身分識別提供者中設定的服務提供者 (SP) 資訊。

### 存取服務提供者資訊

在啟用 SAML 驗證後，SP 資訊會顯示在您的 SAML 設定頁面。此資訊包含身分識別提供者建立 SAML 信任關係所需的所有細節。

### 服務提供者端點

#### SP Entity ID / Audience
**用途**: 唯一識別您的 FastComments 實例作為服務提供者  
**格式**: `https://fastcomments.com/saml/{your-tenant-id}`  
**使用方式**: 在您的 IdP 中將此設定為 Entity ID 或 Audience

此識別碼可確保 SAML 回應是針對您特定的 FastComments 租戶，並防止其他實例接受該 SAML 回應。

#### Assertion Consumer Service (ACS) URL
**用途**: 您的 IdP 在使用者驗證後傳送 SAML 回應的端點  
**格式**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**使用方式**: 在您的 IdP 中將此設定為 ACS URL 或 Reply URL

這是使用者在透過您的身分識別提供者成功驗證後會被導回的位址，並附帶包含使用者資訊的 SAML assertion。

#### SP Metadata URL
**用途**: 以標準 XML 格式提供完整的 SAML 設定  
**格式**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**使用方式**: 有些 IdP 可以使用此 URL 自動匯入設定

metadata URL 含有所有必要的 SP 資訊，採用 XML 格式，便於自動設定相容的身分識別提供者。

#### SAML Login URL
**用途**: 用於啟動針對您租戶的 SAML 驗證的直接連結  
**格式**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**使用方式**: 將使用者直接連結至 SAML 驗證或測試流程

您可以使用此 URL 測試 SAML 驗證，或提供給使用者一個透過 SAML 登入的直接連結。

### SAML Binding Support

FastComments 支援以下 SAML binding：

#### HTTP-POST Binding
- **主要方法**: 最常見的 SAML 回應綁定方式  
- **安全性**: SAML 回應透過 HTTP POST 傳送到 ACS URL  
- **使用方式**: 建議用於正式部署

#### HTTP-Redirect Binding
- **替代方法**: 透過 HTTP redirect 傳送 SAML 回應  
- **限制**: 由於 URL 長度限制，載荷大小有限  
- **使用方式**: 支援但建議優先使用 HTTP-POST

### Name ID Policy

FastComments 在 SAML 請求中設定以下 Name ID policy：

- **預設格式**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **替代格式**: Persistent、Transient、Unspecified（可配置）  
- **需求**: 電子郵件地址被用作主要的使用者識別符

### SAML Request Attributes

在啟動 SAML 驗證時，FastComments 發出的請求具有以下特性：

#### Request Signing
- **狀態**: 可選（可配置）  
- **演算法**: 與所設定的簽章演算法相符  
- **憑證**: 若啟用請求簽章，會使用租戶專屬憑證

#### Requested Attributes
FastComments 在 SAML AuthnRequests 中請求下列屬性：

- **Email**: 作為使用者識別的必需欄位  
- **First Name**: 作為顯示用途的可選欄位  
- **Last Name**: 作為顯示用途的可選欄位  
- **Roles/Groups**: 作為存取控制與權限的可選欄位

### 複製 SP 資訊

SAML 設定頁面提供可點擊的欄位，可自動將 SP 資訊複製到剪貼簿：

1. 點擊任一 SP 資訊欄位（Entity ID、ACS URL 等）  
2. 該值會自動複製到剪貼簿  
3. 將該值貼到您的身分識別提供者設定中  
4. 一個短暫的高亮顯示表示複製成功

這可讓您輕鬆且準確地將 SP 資訊傳送到 IdP，避免輸入錯誤。

### SP Certificate Information

#### Certificate Usage
- **用途**: 加密通訊並驗證 SP 身分  
- **輪替**: 憑證由 FastComments 自動管理  
- **存取**: 公開憑證可透過 metadata URL 取得

#### Certificate Details
- **演算法**: RSA-2048 或更高  
- **有效期**: 憑證會在到期前自動續期  
- **發佈**: 可透過標準的 SAML metadata 取得

### Troubleshooting SP Configuration

如果您的身分識別提供者回報 SP 資訊有問題：

1. **確認 URL**: 確保所有 URL 使用 HTTPS 並包含正確的租戶 ID  
2. **檢查 Metadata**: 使用 metadata URL 驗證設定  
3. **測試連線性**: 確保您的 IdP 能存取 FastComments 的端點  
4. **驗證格式**: 確認您的 IdP 支援該 SP 資訊格式

常見問題包括：
- URL 中的租戶 ID 不正確  
- IdP 與 FastComments 之間的網路連線問題  
- IdP 期望不同的 URL 格式或額外的設定選項