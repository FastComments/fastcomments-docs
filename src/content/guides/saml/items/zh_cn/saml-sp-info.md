当在 FastComments 中启用 SAML 后，系统会自动生成服务提供商（SP）信息，您需要在身份提供商中进行配置。

### 访问服务提供商信息

启用 SAML 身份验证后，SP 信息会显示在您的 SAML 配置页面中。这些信息包括您的身份提供商建立 SAML 信任关系所需的全部详细信息。

### 服务提供商端点

#### SP Entity ID / Audience
**用途**：唯一标识您的 FastComments 实例作为服务提供商
**格式**：`https://fastcomments.com/saml/{your-tenant-id}`
**用法**：在您的 IdP 中将其配置为 Entity ID 或 Audience

此标识符确保 SAML 响应是针对您特定的 FastComments 租户的，并防止 SAML 响应被其他实例接受。

#### Assertion Consumer Service (ACS) URL
**用途**：您的 IdP 在用户身份验证后发送 SAML 响应的端点
**格式**：`https://fastcomments.com/saml/callback/{your-tenant-id}`
**用法**：在您的 IdP 中将其配置为 ACS URL 或 Reply URL

这是用户在您的身份提供商成功完成身份验证后被重定向到的位置，同时会附带包含用户信息的 SAML 断言。

#### SP Metadata URL
**用途**：以标准 XML 格式提供完整的 SAML 配置
**格式**：`https://fastcomments.com/saml/metadata/{your-tenant-id}`
**用法**：某些 IdP 可以使用此 URL 自动导入配置

元数据 URL 以 XML 格式包含所有必要的 SP 信息，便于自动配置兼容的身份提供商。

#### SAML 登录 URL
**用途**：用于为您的租户发起 SAML 身份验证的直接链接
**格式**：`https://fastcomments.com/saml/login/{your-tenant-id}`
**用法**：将用户直接链接到 SAML 身份验证或测试该流程

您可以使用此 URL 测试 SAML 身份验证，或为用户提供通过 SAML 登录的直接链接。

### SAML 绑定支持

FastComments 支持以下 SAML 绑定：

#### HTTP-POST 绑定
- **主要方法**：最常见的 SAML 响应绑定方式
- **安全性**：SAML 响应通过 HTTP POST 发送到 ACS URL
- **用法**：推荐用于生产环境部署

#### HTTP-Redirect 绑定
- **替代方法**：SAML 响应通过 HTTP 重定向发送
- **限制**：由于 URL 长度限制，负载大小受限
- **用法**：受支持，但首选 HTTP-POST

### Name ID 策略

FastComments 在 SAML 请求中配置以下 Name ID 策略：

- **默认格式**：`urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **替代格式**：Persistent、Transient、Unspecified（可配置）
- **要求**：电子邮件地址用作主要用户标识符

### SAML 请求属性

在发起 SAML 身份验证时，FastComments 发送的请求具有以下特征：

#### 请求签名
- **状态**：可选（可配置）
- **算法**：与配置的签名算法一致
- **证书**：如果启用了请求签名，则使用租户专属证书

#### 请求的属性
FastComments 在 SAML AuthnRequests 中请求以下属性：

- **Email**：用户识别所必需
- **First Name**：可选，用于显示目的
- **Last Name**：可选，用于显示目的
- **Roles/Groups**：可选，用于访问控制和权限管理

### 复制 SP 信息

SAML 配置页面提供了可点击的字段，可自动将 SP 信息复制到您的剪贴板：

1. 点击任意 SP 信息字段（Entity ID、ACS URL 等）
2. 该值会自动复制到您的剪贴板
3. 将该值粘贴到您的身份提供商配置中
4. 短暂的高亮显示表示复制成功

这使得将 SP 信息准确传输到您的 IdP 变得轻松，避免输入错误。

### SP 证书信息

#### 证书用途
- **用途**：加密通信并验证 SP 身份
- **轮换**：证书由 FastComments 自动管理
- **访问**：公共证书可通过元数据 URL 获取

#### 证书详情
- **算法**：RSA-2048 或更高
- **有效期**：证书会在到期前自动续期
- **分发**：通过标准 SAML 元数据提供

### SP 配置故障排除

如果您的身份提供商报告 SP 信息存在问题：

1. **验证 URL**：确保所有 URL 使用 HTTPS 并包含正确的租户 ID
2. **检查元数据**：使用元数据 URL 验证配置
3. **测试连接**：确保您的 IdP 可以访问 FastComments 端点
4. **验证格式**：确认您的 IdP 支持 SP 信息格式

常见问题包括：
- URL 中的租户 ID 不正确
- IdP 与 FastComments 之间的网络连接问题
- IdP 期望不同的 URL 格式或额外的配置选项