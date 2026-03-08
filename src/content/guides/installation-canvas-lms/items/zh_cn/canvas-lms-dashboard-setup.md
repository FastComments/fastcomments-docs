#### 导航到 Canvas LTI 配置

登录到您的 FastComments 帐户并转到 <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>。

#### 创建新的 LTI 配置

勾选 **Enable LTI** 复选框。配置字段将出现：

- **Configuration Name** - 可选的标签，用于标识此配置（如果连接多个 Canvas 实例时很有用）。
- **Platform URL** - 您的 Canvas 实例 URL（例如 `https://yourschool.instructure.com`）。您可以先留空，创建开发者密钥后再填写。
- **Client ID** - 现在先留空。您将在 Canvas 中创建开发者密钥后填写它。
- **Deployment ID** - 现在先留空。
- **Comment Style** - 选择 Comments、Collab Chat 或 Both。详情见 [Commenting Styles](#canvas-lms-commenting-styles)。

点击 **Add** 创建配置。

#### 复制配置 URL

保存后，将出现三个 URL：

- **Configuration URL** - 在 Canvas 创建开发者密钥时需要将其粘贴到 Canvas 中。
- **OIDC Login URL** - Canvas 用于 LTI 登录流程的 URL（通过 Configuration URL 自动配置）。
- **Launch URL** - 当学生打开 FastComments 时 Canvas 调用的端点（通过 Configuration URL 自动配置）。

复制 **Configuration URL**。在下一步您将需要它。