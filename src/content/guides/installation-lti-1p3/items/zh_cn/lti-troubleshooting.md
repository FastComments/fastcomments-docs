#### "Registration token not found, expired, or already used"

您注册 URL 中的令牌（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>）仅在 30 分钟内有效且只能使用一次。如果您的 LMS 花费了更长时间，或在注册成功后又重试，令牌会被拒绝。请在 FastComments 的 LTI 1.3 配置页面生成新的 URL 并重新开始。

#### "Platform rejected registration"

您的 LMS 拒绝了注册握手。最常见的原因：

- **Tool already registered with the same client name.** 一些平台（尤其是 D2L）在未删除先前注册项之前会拒绝再次注册名为 “FastComments” 的工具。请在您的 LMS 中移除旧工具，然后重试。
- **Wrong field in the LMS.** 确保您将 URL 粘贴到了 **registration / tool initiation registration endpoint** 字段，而不是 launch URL 或 login URL 字段。
- **The LMS doesn't actually support Dynamic Registration.** 较旧的 Moodle 和 Blackboard 版本虽然宣称支持 LTI 1.3，但仅允许手动配置。请查看您平台的文档。

#### "Failed to fetch platform configuration"

FastComments 无法读取您 LMS 的 openid-configuration 文档。这种情况很少见，通常意味着 LMS 提供了格式错误或无法访问的发现（discovery）URL。请联系您的 LMS 支持。

#### Launch shows "Configuration not found"

要么 FastComments 中的配置被删除了，要么启动来自我们不识别的 `iss`/`client_id` 组合。如果您已删除并重新注册，请指示您的 LMS 删除并重新添加 FastComments 工具，以便获取新的 client_id。

#### Launch shows "Deployment not registered"

您从与首次启动时不同的 Brightspace/Moodle/Blackboard 部署中启动了 FastComments。FastComments 在首次启动时会将 `deployment_id` 固定以作为安全检查。要在同一客户端下添加新部署，请联系支持 —— 我们会将该部署 ID 添加到配置中。

#### Launch shows "Unsupported message_type"

LMS 发送了 FastComments 不处理的 LTI 消息（例如 `LtiSubmissionReviewRequest`）。FastComments 仅支持标准的 resource-link 启动和 deep-linking 流程。如果您需要添加特定的消息类型，请与我们联系。

#### Iframe doesn't resize

大多数 LMS 会自动调整 LTI iframe 的大小。如果您的平台没有，请检查 LMS 的启动设置是否允许该工具向父框架发送 postMessage 事件。FastComments 会发送 Canvas 风格的（`lti.frameResize`）和 IMS 规范的（`org.imsglobal.lti.frameResize`）调整大小消息。