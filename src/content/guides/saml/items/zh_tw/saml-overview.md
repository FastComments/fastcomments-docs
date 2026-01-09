SAML (Security Assertion Markup Language) 是一種基於 XML 的開放標準，用於在各方之間交換身份驗證和授權資料，特別是在身份提供者 (IdP) 與服務提供者 (SP) 之間。

### How SAML Works

SAML 透過允許使用者在其身份提供者上只進行一次驗證，然後存取多個應用程式而無需重複輸入憑證，來實現單一登入 (SSO)。當使用者嘗試存取 FastComments 時：

1. **Authentication Request**: FastComments 將使用者重新導向到您的身份提供者  
2. **User Authentication**: 使用者在您的 IdP（例如 Active Directory、Okta、Azure AD）進行驗證  
3. **SAML Response**: IdP 向 FastComments 發送已簽署的 SAML 聲明  
4. **User Access**: FastComments 驗證該聲明並授予已驗證使用者存取權限

### Benefits of SAML

- **Enhanced Security**: 集中式驗證可降低與密碼相關的安全風險  
- **Improved User Experience**: 使用者只需登入一次即可無縫存取多個應用程式  
- **Compliance**: 有助於滿足存取控制與稽核軌跡的法規要求  
- **Administrative Control**: IT 管理員可維持集中式的使用者管理

### SAML 2.0 Support

FastComments 實作了 SAML 2.0，此為 SAML 標準中被廣泛採用的版本。我們的實作支援：

- HTTP-POST 與 HTTP-Redirect 綁定  
- 已簽署的 SAML 回應與聲明  
- 加密聲明（可選）  
- 多種簽章與摘要演算法  
- 各種名稱識別符格式