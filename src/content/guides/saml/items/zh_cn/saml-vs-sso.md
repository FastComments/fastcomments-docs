---
FastComments 提供 SSO 和 SAML 两种认证方法。了解它们之间的区别有助于为您的组织选择合适的方案。

### Simple/Secure SSO 方案

FastComments 为通过您网站上的评论小部件进行身份验证提供两种不同的 SSO 流程。
这与 SAML 不同，也不需要 SAML。相反，Simple SSO 仅需将一个对象传递给
评论小部件，而 Secure SSO 则在此基础上使用 API 密钥对负载进行哈希处理。

另一方面，SAML 会对整个产品（基于用户权限）进行用户认证，*以及*评论小部件
（如果他们为我们的域启用了第三方 Cookie）。

### SAML 认证

SAML 是一种企业级认证协议，提供更强的安全性和集成能力：

- **Implementation**：需要配置 Identity Provider (IdP) 并交换证书
- **Security**：使用签名的 XML 断言并支持加密
- **Use Case**：适用于已有 SAML 基础设施的企业（Active Directory、Okta 等）
- **Setup Complexity**：设置更复杂——需要 IdP 配置和证书管理
- **Enterprise Features**：高级角色映射、集中用户管理、审计追踪

### When to Choose SAML

如果您的组织符合以下情况，请考虑使用 SAML 认证：

- 已经使用兼容 SAML 的身份提供商（Okta、Azure AD、ADFS 等）
- 需要企业级的安全性和合规性
- 需要集中式用户管理和访问控制
- 有多个应用使用 SAML 进行认证
- 需要详细的审计追踪和安全报告

### When to Choose Simple or Secure SSO

如果您符合以下情况，我们面向小部件的 SSO 解决方案可能已足够：

- 拥有自定义认证系统
- 需要快速实现并尽量减少设置工作
- 不需要企业身份提供商集成
- 想直接从您的应用控制用户数据
- 安全需求较为简单

Simple 和 Secure SSO 常用于在线门户、博客等场景，用户已经通过您的网站或应用拥有账户，*但不一定使用 SAML*。

---