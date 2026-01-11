The FastComments SDK 提供两类 API 端点：

### PublicAPI - 客户端安全端点

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- 不需要 API 密钥
- 可以使用 SSO 令牌进行身份验证
- 对每个用户/设备进行速率限制
- 适用于面向终端用户的应用

**Example use case**: 在您的 iOS 应用中获取和创建评论

### DefaultAPI - 服务器端端点

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- 需要您的 FastComments API 密钥
- 仅应从服务器端代码调用
- 提供对您的 FastComments 数据的完全访问
- 按租户限流

**Example use case**: 管理操作、批量数据导出、审核工具

**IMPORTANT**: 切勿在客户端代码中暴露您的 API 密钥。API 密钥只应在服务器端使用。