When SAML is enabled in FastComments, the system automatically generates Service Provider (SP) information that you need to configure in your identity provider.
当在 FastComments 中启用 SAML 时，系统会自动生成您需要在身份提供者中配置的服务提供者 (SP) 信息。

### Accessing Service Provider Information
### 访问服务提供者信息

The SP information is displayed in your SAML configuration page after enabling SAML authentication. This information includes all the details your identity provider needs to establish the SAML trust relationship.
启用 SAML 身份验证后，SP 信息会显示在您的 SAML 配置页面中。此信息包含身份提供者建立 SAML 信任关系所需的所有详细信息。

### Service Provider Endpoints
### 服务提供者端点

#### SP Entity ID / Audience
#### SP 实体 ID / 受众
**Purpose**: Uniquely identifies your FastComments instance as a service provider  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Usage**: Configure this as the Entity ID or Audience in your IdP

**目的**：唯一标识您的 FastComments 实例作为服务提供者  
**格式**：`https://fastcomments.com/saml/{your-tenant-id}`  
**用法**：在您的 IdP 中将此配置为实体 ID 或受众

This identifier ensures that SAML responses are intended for your specific FastComments tenant and prevents SAML responses from being accepted by other instances.
该标识确保 SAML 响应针对您的特定 FastComments 租户，并防止其他实例接受这些 SAML 响应。

#### Assertion Consumer Service (ACS) URL
#### Assertion Consumer Service (ACS) URL
**Purpose**: The endpoint where your IdP sends SAML responses after user authentication  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Usage**: Configure this as the ACS URL or Reply URL in your IdP

**目的**：您的 IdP 在用户身份验证后发送 SAML 响应的端点  
**格式**：`https://fastcomments.com/saml/callback/{your-tenant-id}`  
**用法**：在您的 IdP 中将此配置为 ACS URL 或回复 URL

This is where users are redirected after successful authentication with your identity provider, along with the SAML assertion containing user information.
这是用户在通过身份提供者成功验证后被重定向到的位置，同时会随附包含用户信息的 SAML 断言。

#### SP Metadata URL
#### SP 元数据 URL
**Purpose**: Provides complete SAML configuration in standard XML format  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Usage**: Some IdPs can automatically import configuration using this URL

**目的**：以标准 XML 格式提供完整的 SAML 配置  
**格式**：`https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**用法**：某些 IdP 可以使用此 URL 自动导入配置

The metadata URL contains all necessary SP information in XML format, making it easy to configure compatible identity providers automatically.
元数据 URL 以 XML 格式包含所有必要的 SP 信息，便于自动配置兼容的身份提供者。

#### SAML Login URL
#### SAML 登录 URL
**Purpose**: Direct link to initiate SAML authentication for your tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Usage**: Link users directly to SAML authentication or test the flow

**目的**：用于为您的租户直接发起 SAML 身份验证的链接  
**格式**：`https://fastcomments.com/saml/login/{your-tenant-id}`  
**用法**：直接将用户链接到 SAML 身份验证或测试流程

You can use this URL to test SAML authentication or provide users with a direct link to sign in via SAML.
您可以使用此 URL 测试 SAML 身份验证或为用户提供通过 SAML 登录的直接链接。

### SAML Binding Support
### SAML 绑定支持

FastComments supports the following SAML bindings:
FastComments 支持以下 SAML 绑定：

#### HTTP-POST Binding
#### HTTP-POST 绑定
- **Primary Method**: Most common binding for SAML responses  
- **Security**: SAML response is sent via HTTP POST to the ACS URL  
- **Usage**: Recommended for production deployments

- **主要方法**：最常见的 SAML 响应绑定  
- **安全性**：SAML 响应通过 HTTP POST 发送到 ACS URL  
- **用法**：推荐用于生产部署

#### HTTP-Redirect Binding
#### HTTP-Redirect 绑定
- **Alternative Method**: SAML response sent via HTTP redirect  
- **Limitations**: Limited payload size due to URL length restrictions  
- **Usage**: Supported but HTTP-POST is preferred

- **替代方法**：SAML 响应通过 HTTP 重定向发送  
- **限制**：由于 URL 长度限制，载荷大小受限  
- **用法**：支持但推荐使用 HTTP-POST

### Name ID Policy
### Name ID 策略

FastComments configures the following Name ID policy in SAML requests:
FastComments 在 SAML 请求中配置以下 Name ID 策略：

- **Default Format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternative Formats**: Persistent, Transient, Unspecified (configurable)  
- **Requirement**: The email address is used as the primary user identifier

- **默认格式**：`urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **备选格式**：Persistent、Transient、Unspecified（可配置）  
- **要求**：电子邮件地址用作主要用户标识符

### SAML Request Attributes
### SAML 请求属性

When initiating SAML authentication, FastComments sends requests with these characteristics:
在发起 SAML 身份验证时，FastComments 会发送具有以下特征的请求：

#### Request Signing
#### 请求签名
- **Status**: Optional (configurable)  
- **Algorithm**: Matches configured signature algorithm  
- **Certificate**: Uses tenant-specific certificate if request signing is enabled

- **状态**：可选（可配置）  
- **算法**：与配置的签名算法相匹配  
- **证书**：如果启用请求签名，则使用租户特定的证书

#### Requested Attributes
#### 请求的属性
FastComments requests the following attributes in SAML AuthnRequests:

FastComments 在 SAML AuthnRequests 中请求以下属性：

- **Email**: Required for user identification  
- **First Name**: Optional for display purposes  
- **Last Name**: Optional for display purposes  
- **Roles/Groups**: Optional for access control and permissions

- **电子邮件**：用于用户识别，必填  
- **名**：用于展示，可选  
- **姓**：用于展示，可选  
- **角色/组**：用于访问控制和权限，可选

### Copying SP Information
### 复制 SP 信息

The SAML configuration page provides clickable fields that automatically copy SP information to your clipboard:

SAML 配置页面提供可点击字段，可将 SP 信息自动复制到您的剪贴板：

1. Click any SP information field (Entity ID, ACS URL, etc.)  
2. The value is automatically copied to your clipboard  
3. Paste the value into your identity provider configuration  
4. A brief highlight indicates successful copying

1. 单击任何 SP 信息字段（实体 ID、ACS URL 等）  
2. 值会自动复制到您的剪贴板  
3. 将该值粘贴到您的身份提供者配置中  
4. 短暂高亮表示复制成功

This makes it easy to accurately transfer the SP information to your IdP without typing errors.
这使您能够轻松且准确地将 SP 信息传输到 IdP，避免输入错误。

### SP Certificate Information
### SP 证书信息

#### Certificate Usage
#### 证书用途
- **Purpose**: Encrypts communications and verifies SP identity  
- **Rotation**: Certificates are automatically managed by FastComments  
- **Access**: Public certificates are available via the metadata URL

- **目的**：加密通信并验证 SP 身份  
- **轮换**：证书由 FastComments 自动管理  
- **访问**：可通过元数据 URL 获取公用证书

#### Certificate Details
#### 证书详情
- **Algorithm**: RSA-2048 or higher  
- **Validity**: Certificates are automatically renewed before expiration  
- **Distribution**: Available through standard SAML metadata

- **算法**：RSA-2048 或更高  
- **有效期**：证书将在到期前自动续期  
- **分发**：通过标准 SAML 元数据提供

### Troubleshooting SP Configuration
### SP 配置故障排除

If your identity provider reports issues with SP information:

如果您的身份提供者报告 SP 信息问题：

1. **Verify URLs**: Ensure all URLs use HTTPS and include the correct tenant ID  
2. **Check Metadata**: Use the metadata URL to verify configuration  
3. **Test Connectivity**: Ensure your IdP can reach FastComments endpoints  
4. **Validate Format**: Confirm your IdP supports the SP information format

1. **验证 URL**：确保所有 URL 使用 HTTPS 并包含正确的租户 ID  
2. **检查元数据**：使用元数据 URL 验证配置  
3. **测试连接性**：确保您的 IdP 能够访问 FastComments 端点  
4. **验证格式**：确认您的 IdP 支持 SP 信息的格式

Common issues include:
常见问题包括：
- Incorrect tenant ID in URLs  
- Network connectivity problems between IdP and FastComments  
- IdP expecting different URL formats or additional configuration options

- URL 中的租户 ID 不正确  
- IdP 与 FastComments 之间的网络连接问题  
- IdP 期望不同的 URL 格式或其他配置选项