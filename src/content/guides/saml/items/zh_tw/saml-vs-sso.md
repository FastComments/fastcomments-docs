FastComments 提供 SSO 和 SAML 認證。了解差異有助於您為組織選擇合適的方法。

### Simple/Secure SSO 流程

FastComments 提供兩種不同的 SSO 流程，用於透過您的網站在評論元件中進行驗證。這與 SAML 不同，且不需要 SAML。Simple SSO 僅需將一個物件傳給評論元件，而 Secure SSO 則在此基礎上使用 API 金鑰對有效載荷進行雜湊。

SAML 則會對整個產品進行使用者驗證（基於其權限）以及評論元件的驗證（如果使用者為我們的網域啟用了第三方 Cookie）。

### SAML Authentication

SAML 是一種企業級的驗證協議，提供更完整的安全性與整合能力：

- **Implementation**: 需要設定身分提供者 (IdP) 並進行憑證交換
- **Security**: 使用已簽署的 XML 斷言並支援加密
- **Use Case**: 適合已有 SAML 基礎設施（Active Directory、Okta 等）的企業
- **Setup Complexity**: 較為複雜 - 需要 IdP 設定與憑證管理
- **Enterprise Features**: 進階的角色對應、集中式用戶管理、稽核紀錄

### When to Choose SAML

若您的組織符合下列情況，請考慮使用 SAML 認證：

- 已在使用相容 SAML 的身分提供者（Okta、Azure AD、ADFS 等）
- 需要企業級的安全與合規性
- 需要集中式的用戶管理與存取控制
- 有多個應用程式使用 SAML 進行驗證
- 需要詳細的稽核紀錄與安全報告

### When to Choose Simple or Secure SSO

如果您符合下列情況，我們以元件為中心的 SSO 解決方案可能就足夠：

- 擁有自訂的驗證系統
- 需要快速實作且設定最少
- 不需要整合企業的身分提供者
- 希望直接從您的應用控制使用者資料
- 只有較簡單的安全性需求

Simple 和 Secure SSO 常用於線上入口網站、部落格等情境，使用者已經有透過您的網站或應用程式的帳號，但不一定使用 SAML。