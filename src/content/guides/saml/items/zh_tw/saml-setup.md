在 FastComments 中設定 SAML 驗證需要同時在您的管理後台進行配置，並在您的身份提供者（IdP）端完成設定。

### 前置需求

在配置 SAML 之前，請確保您具備：

- FastComments Flex 或 Pro 方案（Creators 方案不提供 SAML 功能）
- FastComments 帳戶的管理員權限
- 身份提供者的管理員權限
- 您 IdP 的 SAML 中繼資料或憑證資訊

### 存取 SAML 配置

1. 登入您的 [FastComments 管理後台](https://fastcomments.com/auth/my-account)
2. 在左側邊欄導覽至 **API/SSO Settings**
3. 點擊 **SAML Config** 按鈕

如果您沒有看到 SAML Config 按鈕，請確認：
- 您的帳戶具有所需的方案（Flex 或 Pro）
- 您具有管理員權限
- 您的使用者具有 API Admin 或 Admin Admin 角色

### 基本 SAML 配置

#### 啟用 SAML 驗證

1. 勾選 **Enable SAML Authentication** 核取方塊
2. 這將為您的租戶啟用 SAML，並使配置欄位可用

#### 必填欄位

**IdP Single Sign-On URL** *(必填)*
- 使用者將被重新導向以進行驗證的 URL
- 通常由您的身份提供者提供
- 範例：`https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(必填)*
- 來自您身份提供者的公開憑證
- 用於驗證 SAML 回應的真實性
- 必須包含完整的憑證，包括 BEGIN/END 標記
- 範例格式：
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### 選填欄位

**IdP Entity ID / Issuer**
- 識別您的身份提供者
- 若留空，預設為您的 FastComments URL
- 應與您 IdP 中配置的簽發者一致

### 進階配置

#### 安全性設定

**Signature Algorithm**
- 預設為 SHA-256（建議）
- 選項：SHA-1、SHA-256、SHA-512
- 應與您 IdP 的配置一致

**Digest Algorithm**
- 預設為 SHA-256（建議）
- 用於 SAML 回應中的摘要計算
- 應與您 IdP 的配置一致

**Name ID Format**
- 預設為 Email Address 格式
- 決定使用者識別碼的格式化方式
- 常見選項：Email Address、Persistent、Transient

#### 加密（選填）

**Private Key for Decryption**
- 僅在您的 IdP 加密 SAML 斷言（assertion）時才需要
- 貼上您用於解密的私密金鑰
- 大多數部署不需要加密斷言

### 儲存配置

1. 檢查所有設定的正確性
2. 點擊 **Save SAML Configuration**
3. 系統將驗證您的配置
4. 若成功，您將看到確認訊息

### 後續步驟

儲存您的 FastComments SAML 配置後：

1. 使用服務提供者（Service Provider）資訊配置您的身份提供者
2. 測試驗證流程
3. 根據需要設定使用者角色與權限

啟用 SAML 後，您的 IdP 配置所需的服務提供者資訊將會顯示出來。