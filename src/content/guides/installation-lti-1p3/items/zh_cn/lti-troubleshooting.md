#### "注册令牌未找到、已过期或已被使用"

注册 URL 中的令牌有效期为 30 分钟且仅可使用一次。如果你的 LMS 用时超过该期限，或者在注册成功后再次尝试注册，令牌将被拒绝。请在 FastComments LTI 1.3 配置页面生成新的 URL 并重新开始。

#### "Platform rejected registration"

你的 LMS 拒绝了注册握手。最常见的原因：

- **Tool already registered with the same client name.** 有些平台（尤其是 D2L）会拒绝再次注册 “FastComments”，直到删除之前的注册项。请在你的 LMS 中删除旧的工具，然后重试。
- **Wrong field in the LMS.** 确保你将 URL 粘贴到 **registration / tool initiation registration endpoint** 字段，而不是 launch URL 或 login URL 字段。
- **The LMS doesn't actually support Dynamic Registration.** 较旧的 Moodle 和 Blackboard 版本宣称支持 LTI 1.3，但只允许手动配置。查看你们平台的文档。

#### "Failed to fetch platform configuration"

FastComments 无法读取你 LMS 的 openid-configuration 文档。这种情况很少见，通常意味着 LMS 提供的 discovery URL 格式错误或不可访问。请联系你们的 LMS 支持。

#### Launch shows "Configuration not found"

要么 FastComments 中的配置被删除，要么启动请求来自我们不认识的 `iss`/`client_id` 配对。如果你删除并重新注册了，请指示你的 LMS 移除并重新添加 FastComments 工具，以便它获取新的 client_id。

#### Launch shows "Deployment not registered"

你从与最初不同的 Brightspace/Moodle/Blackboard 部署中启动了 FastComments。FastComments 会在首次启动时固定 `deployment_id` 作为安全检查。要在同一客户端下添加新的部署，请联系支持 - 我们会将该 deployment ID 添加到配置中。

#### Launch shows "Unsupported message_type"

LMS 发送了 FastComments 无法处理的 LTI 消息（例如 `LtiSubmissionReviewRequest`）。FastComments 仅支持标准的 resource-link 启动和 deep-linking 流程。如果你需要添加特定的消息类型，请联系我们。

#### Iframe doesn't resize

大多数 LMS 会自动调整 LTI iframe 的大小。如果你的 LMS 不会，请检查 LMS 的启动设置是否允许该工具向父框架发送 postMessage 事件。FastComments 会发送 Canvas 风格（`lti.frameResize`）和 IMS 规范（`org.imsglobal.lti.frameResize`）两种调整大小的消息。

---