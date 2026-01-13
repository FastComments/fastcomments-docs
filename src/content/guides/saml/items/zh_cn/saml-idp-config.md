配置 FastComments 的 SAML 之后，您需要在您的身份提供者中将 FastComments 设置为服务提供者。

### 一般 IdP 配置

大多数身份提供者需要以下信息以将 FastComments 添加为 SAML 应用：

#### 必需的服务提供者信息

这些值会在您的 FastComments SAML 配置页面自动生成并显示：

**SP 实体 ID / 受众**
- 格式： `https://fastcomments.com/saml/{your-tenant-id}`
- 这用于唯一标识您的 FastComments 实例

**断言消费者服务 (ACS) URL**
- 格式： `https://fastcomments.com/saml/callback/{your-tenant-id}`
- 这是您的 IdP 在身份验证后发送 SAML 响应的位置

**SP 元数据 URL** *(如果您的 IdP 支持)*
- 格式： `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- 以 XML 格式提供完整的 SAML 配置

**SAML 登录 URL**
- 格式： `https://fastcomments.com/saml/login/{your-tenant-id}`
- 用于启动 SAML 认证的直接链接

### 必需的 SAML 属性

配置您的身份提供者以在 SAML 响应中发送这些属性：

#### 基本属性

**电子邮箱地址** *(必需)*
- **属性名称**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **用途**：唯一的用户标识和通知
- **格式**：有效的电子邮件地址

#### 可选属性

**名**
- **属性名称**：`firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **用途**：用户显示名称

**姓**
- **属性名称**：`lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **用途**：用户显示名称

**角色** *(对访问控制很重要)*
- **属性名称**：`roles`, `groups`, `memberOf`, or custom attribute names
- **用途**：FastComments 的角色分配和权限
- **格式**：角色字符串数组或逗号分隔值

### 常见身份提供者配置

#### Microsoft Azure AD

1. **添加企业应用**
   - 搜索 "FastComments" 或创建自定义 SAML 应用
   - 使用 FastComments 提供的 SP 信息

2. **配置属性**
   - 电子邮件： `user.mail` 或 `user.userprincipalname`
   - 名： `user.givenname`
   - 姓： `user.surname`
   - 角色： `user.assignedroles` 或 目录组

#### Okta

1. **创建 SAML 应用**
   - 使用 "Create New App" 并选择 SAML 2.0
   - 使用 FastComments 的 SP 信息进行配置

2. **属性声明**
   - 电子邮件： `user.email`
   - 名： `user.firstName`
   - 姓： `user.lastName`
   - 角色： `user.groups` 或 自定义属性

#### Google Workspace

1. **添加 SAML 应用**
   - 转到 Apps > Web and mobile apps > Add App > Add custom SAML app
   - 使用 FastComments 的 SP 信息进行配置

2. **属性映射**
   - 电子邮件： 主邮件
   - 名： 名
   - 姓： 姓
   - 角色： 群组或自定义属性

#### Active Directory Federation Services (ADFS)

1. **添加依赖方信任**
   - 使用 FastComments 元数据 URL 或手动配置
   - 按提供的信息配置 SP

2. **声明规则**
   - 电子邮件：Email Address 声明
   - 名称：Name ID 声明
   - 角色：组成员关系或自定义声明

### 属性名称灵活性

FastComments 接受来自多个属性名称的角色信息以适应不同的 IdP 配置：

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

这种灵活性确保与各种身份提供者兼容，而无需特定的属性命名约定。

### 测试您的配置

在配置身份提供者后：

1. 保存 IdP 配置
2. 使用专用测试用户账号进行测试
3. 验证属性是否被正确发送
4. 检查角色是否被正确映射
5. 确保身份验证流程成功完成

大多数身份提供者在部署到生产用户之前提供 SAML 测试工具以验证配置。