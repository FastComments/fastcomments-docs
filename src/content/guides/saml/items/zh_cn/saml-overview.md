SAML (Security Assertion Markup Language) 是一种基于 XML 的开放标准，用于在各方之间交换身份验证和授权数据，尤其是在身份提供者 (IdP) 与服务提供者 (SP) 之间。

### How SAML Works

SAML 通过允许用户在其身份提供者上进行一次身份验证，然后访问多个应用程序而无需重新输入凭据，从而实现单点登录 (SSO)。当用户尝试访问 FastComments 时：

1. **Authentication Request**: FastComments 将用户重定向到您的身份提供者
2. **User Authentication**: 用户在您的 IdP（例如 Active Directory、Okta、Azure AD）上进行身份验证
3. **SAML Response**: IdP 向 FastComments 发送签名的 SAML 断言
4. **User Access**: FastComments 验证该断言并授予经过身份验证的用户访问权限

### Benefits of SAML

- **Enhanced Security**: 集中式身份验证降低了与密码相关的安全风险
- **Improved User Experience**: 用户只需登录一次即可无缝访问多个应用程序
- **Compliance**: 有助于满足访问控制和审计跟踪的法规要求
- **Administrative Control**: IT 管理员保持集中式用户管理

### SAML 2.0 Support

FastComments 实现了 SAML 2.0，这是 SAML 标准中被广泛采用的版本。我们的实现支持：

- HTTP-POST and HTTP-Redirect 绑定
- 签名的 SAML 响应和断言
- 加密断言（可选）
- 多种签名和摘要算法
- 各种名称标识符格式