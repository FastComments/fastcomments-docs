---
#### Sakai

Sakai 在支持 LTI Advantage 的版本中支持 LTI 1.3 动态注册。从 **管理工作区**：

1. 以 Sakai 管理员身份登录并打开 **管理工作区**。
2. 选择 **外部工具** > **安装 LTI 1.3 工具**。
3. 粘贴 FastComments 注册 URL 并提交。
4. 在握手完成后批准该工具。

该工具随后会出现在 **外部工具** 下，站点维护者可以将其添加到站点中。

#### Schoology

Schoology 企业实例支持 LTI 1.3，但动态注册的可用性因部署而异。请与您的 Schoology 客户经理联系。

如果您的 Schoology 实例不支持动态注册，您需要使用以下端点手动配置集成：

- **OIDC 登录 URL**: `https://fastcomments.com/lti/v1p3/login`
- **目标链接 URL**: `https://fastcomments.com/lti/v1p3/launch`
- **公钥集 URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **重定向 URL**: `https://fastcomments.com/lti/v1p3/launch`

在 Schoology 向您提供 Client ID 和 Deployment ID 之后，联系 FastComments 支持以在您的租户上注册该配置。

#### Other LTI 1.3 Platforms

任何遵循 IMS LTI 1.3 Advantage 规范的 LMS 应该都可以使用相同的注册 URL。请查找标有 "动态注册"、"工具注册 URL"、"工具启动注册端点" 或类似名称的设置。

如果您的平台仅支持手动 LTI 1.3 设置，请使用上述 Schoology 部分列出的四个端点，并联系支持团队以完成配置。

---