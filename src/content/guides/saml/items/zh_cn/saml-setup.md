在 FastComments 中设置 SAML 身份验证需要在您的管理员仪表板和身份提供商两端进行配置。

### 先决条件

在配置 SAML 之前，请确保您具备：

- FastComments 的 Flex 或 Pro 计划（Creators 计划不支持 SAML）
- 对您的 FastComments 帐户的管理员访问权限
- 对您的身份提供商的管理员访问权限
- 您 IdP 的 SAML 元数据或证书信息

### 访问 SAML 配置

1. 登录到您的 [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. 在左侧边栏导航到 **API/SSO 设置**
3. 点击 **SAML 配置** 按钮

如果看不到 **SAML 配置** 按钮，请确认：
- 您的账户包含所需套餐（Flex 或 Pro）
- 您具有管理员权限
- 您的用户具有 API Admin 或 Admin Admin 角色

### 基本 SAML 配置

#### 启用 SAML 身份验证

1. 选中 **启用 SAML 身份验证** 复选框
2. 这将为您的租户启用 SAML，并使配置字段可用

#### 必填字段

**IdP Single Sign-On URL** *(必填)*
- 用户将被重定向以进行身份验证的 URL
- 通常由您的身份提供商提供
- 示例: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(必填)*
- 来自您身份提供商的公钥证书
- 用于验证 SAML 响应的真实性
- 必须包含带有 BEGIN/END 标记的完整证书
- 示例格式：
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### 可选字段

**IdP Entity ID / Issuer**
- 标识您的身份提供商
- 如果留空，则默认为您的 FastComments URL
- 应与在您的 IdP 中配置的发行者匹配

### 高级配置

#### 安全设置

**Signature Algorithm**
- 默认使用 SHA-256（推荐）
- 可选：SHA-1、SHA-256、SHA-512
- 应与您的 IdP 配置相匹配

**Digest Algorithm**
- 默认使用 SHA-256（推荐）
- 用于 SAML 响应中的摘要计算
- 应与您的 IdP 配置相匹配

**Name ID Format**
- 默认使用电子邮件地址格式
- 决定用户标识符的格式
- 常见选项：电子邮件地址、Persistent、Transient

#### 加密（可选）

**Private Key for Decryption**
- 仅在您的 IdP 对 SAML 断言进行加密时需要
- 粘贴用于解密的私钥
- 大多数部署不需要断言加密

### 保存配置

1. 检查所有设置是否准确
2. 点击 **保存 SAML 配置**
3. 系统将验证您的配置
4. 如果成功，您将看到确认消息

### 下一步

在保存您的 FastComments SAML 配置后：

1. 使用服务提供者信息配置您的身份提供商
2. 测试身份验证流程
3. 按需设置用户角色和权限

一旦启用 SAML，用于您 IdP 配置的服务提供者信息将会显示。