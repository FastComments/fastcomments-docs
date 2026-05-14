#### 导航到 LTI 1.3 配置

登录 FastComments 并转到 <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">您的 LTI 1.3 配置页面</a>。

如果您的账户尚未具有 LTI 访问权限，您会看到 "LTI not enabled for this account" - 请联系支持以在您的计划中启用它。

#### 选择平台（可选）

在 **生成动态注册 URL** 下，使用 **Platform** 下拉菜单告诉 FastComments 您正在连接哪个 LMS：

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- 其他 LTI 1.3 平台

您也可以将其保留为 **自动检测**。在注册过程中，平台信息会从您的 LMS 的 openid-configuration 中读取；下拉菜单仅用于为生成的配置提供显示标签的初始值。

#### 生成 URL

单击 **Generate URL**。FastComments 会创建一个一次性注册令牌并显示一个类似于以下的 URL：

`https://fastcomments.com/lti/v1p3/register/<long-token>`

复制它。该 URL：

- 是 **一次性使用** — 一旦您的 LMS 成功调用，令牌即被使用。
- 在未使用时会在 **30 分钟** 后到期。
- 应当保密 — 在这 30 分钟内，任何拥有该 URL 的人都可以在您的租户下注册一个工具。

#### 现有配置

注册成功完成后，新配置会显示在同一页面的 **Existing Configurations** 表中，包含其 Platform、Issuer、Client ID 和 Status。若需取消注册，您可以从该表中删除配置。