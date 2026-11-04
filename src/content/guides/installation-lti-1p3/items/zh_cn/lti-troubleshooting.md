#### “未找到注册令牌、令牌已过期或已被使用”

注册 URL 中的令牌（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此获取</a>）有效期为 30 分钟，且只能使用一次。如果您的 LMS 操作耗时超过该时限，或者注册成功后进行了重试，该令牌将被拒绝。请在 FastComments LTI 1.3 配置页面生成一个新的 URL 并重新开始。

#### “平台拒绝注册”

您的 LMS 拒绝了注册握手。最常见的原因包括：

- **已存在使用相同客户端名称注册的工具。** 某些平台（尤其是 D2L）在删除之前的注册之前，会拒绝第二次注册 "FastComments"。请先在您的 LMS 中移除旧工具，然后重试。
- **LMS 中填错了字段。** 请确保您将 URL 粘贴到 **registration / tool initiation registration endpoint** 字段中，而不是启动 URL 或登录 URL 字段。
- **LMS 实际上不支持动态注册。** 较旧版本的 Moodle 和 Blackboard 虽然宣称支持 LTI 1.3，但只允许手动配置。请查阅您平台的文档。

#### “获取平台配置失败”

FastComments 无法读取您 LMS 的 openid-configuration 文档。这种情况很少见，通常意味着 LMS 提供的发现 URL 格式错误或无法访问。请联系您的 LMS 支持团队。

#### 启动时显示 "Configuration not found"

可能是 FastComments 中的配置已被删除，或者启动请求来自我们无法识别的 `iss`/`client_id` 组合。如果您删除并重新注册了配置，请指示您的 LMS 移除并重新添加 FastComments 工具，以便它获取新的 client_id。

#### 启动时显示 "Deployment not registered"

您是从与首次启动时不同的 Brightspace/Moodle/Blackboard 部署中启动 FastComments 的。作为安全检查，FastComments 会在首次启动时固定 `deployment_id`。要在同一客户端下添加新部署，请联系支持团队——我们会将该部署 ID 添加到配置中。

#### 启动时显示 "Unsupported message_type"

LMS 发送了 FastComments 不处理的 LTI 消息（例如 `LtiSubmissionReviewRequest`）。FastComments 仅支持标准的资源链接启动和深度链接流程。如果您需要添加特定的消息类型，请与我们联系。

#### Iframe 无法调整大小

大多数 LMS 会自动调整 LTI iframe 的大小。如果您的 LMS 不会，请检查 LMS 的启动设置是否允许该工具向父框架发送 postMessage 事件。FastComments 会同时发送 Canvas 风格的（`lti.frameResize`）和 IMS 规范的（`org.imsglobal.lti.frameResize`）调整大小消息。