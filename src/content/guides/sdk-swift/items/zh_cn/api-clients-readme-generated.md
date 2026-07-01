FastComments SDK 提供了三个 API 客户端：

### PublicAPI - 客户端安全方法

`PublicAPI` 包含可安全从客户端代码（iOS/macOS 应用）调用的方法。这些方法：
- 不需要 API 密钥
- 可以使用 SSO 令牌进行身份验证
- 按用户/设备进行限流
- 适用于面向最终用户的应用程序

**示例用例**：在 iOS 应用中获取和创建评论

### DefaultAPI - 服务器端方法

`DefaultAPI` 包含需要 API 密钥的已认证方法。这些方法：
- 需要您的 FastComments API 密钥
- 只能从服务器端代码调用
- 提供对您的 FastComments 数据的完整访问
- 按租户进行限流

**示例用例**：管理操作、大批量数据导出、用户管理

### ModerationAPI - 版主仪表盘方法

`ModerationAPI` 提供了一个功能丰富的实时快速审核 API 套件。每个 `ModerationAPI` 方法都接受一个 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 Cookie 进行身份验证。

**示例用例**：为您社区的版主构建审核体验

**重要提示**：切勿在客户端代码中暴露您的 API 密钥。API 密钥应仅在服务器端使用。