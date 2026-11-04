FastComments SDK 提供两种类型的 API 端点：

### PublicAPI - 客户端安全端点

`PublicAPI` 包含可以安全地从客户端代码（iOS/macOS 应用）调用的端点。这些端点：
- 不需要 API 密钥
- 可以使用 SSO 令牌进行身份验证
- 按用户/设备进行速率限制
- 适合面向最终用户的应用程序

**示例用例**：在您的 iOS 应用中获取和创建评论

### DefaultAPI - 服务端端点

`DefaultAPI` 包含需要 API 密钥的已验证端点。这些端点：
- 需要您的 FastComments API 密钥
- 只应从服务端代码调用
- 提供对您的 FastComments 数据的完全访问权限
- 按租户进行速率限制

**示例用例**：管理操作、批量数据导出、审核工具

**重要提示**：切勿在客户端代码中暴露您的 API 密钥。API 密钥只应在服务端使用。