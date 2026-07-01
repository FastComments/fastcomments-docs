### 401 未授权错误

如果在使用已认证的 API 时遇到 401 错误：

1. **检查您的 API 密钥**：确保使用来自 FastComments 仪表板的正确 API 密钥
2. **验证租户 ID**：确保租户 ID 与您的账户匹配
3. **API 密钥格式**：API 密钥应在共享配置中设置为 `x-api-key` 头部：

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **使用错误的 API**：确保在已认证调用中使用 `DefaultAPI`（而不是 `PublicAPI`）  

### SSO 令牌问题

如果 SSO 令牌无法正常工作：

1. **在生产环境中使用安全模式**：始终使用 `FastCommentsSSO.createSecure()` 并传入您的 API 密钥进行生产环境部署
2. **仅限服务器端**：在服务器上生成安全的 SSO 令牌，切勿将 API 密钥暴露给客户端
3. **检查用户数据**：确保提供所有必填字段（id、email、username）
4. **令牌过期**：安全的 SSO 令牌包含时间戳，可能会过期。根据需要生成新的令牌。  

### SSL/TLS 错误

如果遇到 SSL/TLS 错误：

1. 确保您的应用的 Info.plist 允许对 fastcomments.com 的 HTTPS 连接
2. 检查是否未使用可能阻止连接的 App Transport Security 异常