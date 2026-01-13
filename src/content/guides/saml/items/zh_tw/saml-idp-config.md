在 FastComments 中設定 SAML 後，您需要在您的身份提供者中將 FastComments 設定為服務提供者 (Service Provider)。

### 一般 IdP 設定

大多數身份提供者需要以下資訊來將 FastComments 新增為 SAML 應用程式：

#### 必要的服務提供者資訊

這些值會自動產生並顯示在您的 FastComments SAML 設定頁面中：

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- 這會唯一識別您的 FastComments 實例

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- 您的 IdP 在驗證後將 SAML 回應傳送到此處

**SP Metadata URL** *(if supported by your IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- 以 XML 格式提供完整的 SAML 設定

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- 啟動 SAML 驗證的直接連結

### 所需的 SAML 屬性

請將您的身份提供者設定為在 SAML 回應中傳送這些屬性：

#### 重要屬性

**Email Address** *(Required)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: 用於唯一使用者識別與通知
- **Format**: 有效的電子郵件地址

#### 可選屬性

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: 使用者顯示名稱

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: 使用者顯示名稱

**Roles** *(Important for access control)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: FastComments 角色指派與權限
- **Format**: 角色字串陣列或以逗號分隔的值

### 常見的身份提供者設定

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - 搜尋 "FastComments" 或建立自訂 SAML 應用程式
   - 使用 FastComments 提供的 SP 資訊

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - 使用 "Create New App" 並選擇 SAML 2.0
   - 使用 FastComments SP 資訊進行設定

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - 前往 Apps > Web and mobile apps > Add App > Add custom SAML app
   - 使用 FastComments SP 資訊進行設定

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - 使用 FastComments metadata URL 或手動設定
   - 根據提供的資訊設定 SP

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### 屬性名稱彈性

為了配合不同的 IdP 設定，FastComments 接受多種屬性名稱來傳遞角色資訊：

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

此彈性確保與各種身份提供者相容，而不需特定的屬性命名慣例。

### 測試您的設定

在設定身份提供者後：

1. 儲存 IdP 設定
2. 使用專用的測試使用者帳號進行測試
3. 驗證屬性是否正確傳送
4. 檢查角色是否正確映射
5. 確保驗證流程成功完成

大多數身份提供者提供 SAML 測試工具，可在部署給生產使用者前驗證設定。