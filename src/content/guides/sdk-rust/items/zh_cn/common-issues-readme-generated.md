### 401 未授权错误

如果在使用需要身份验证的 API 时遇到 401 错误：

1. **检查你的 API key**：确保你正在使用来自 FastComments dashboard 的正确 API key
2. **验证 tenant ID**：确保 tenant ID 与您的帐户匹配
3. **API key 格式**：API key 应在 Configuration 中传递：

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO 令牌问题

如果 SSO 令牌 无法正常工作：

1. **在生产环境中使用安全模式**：始终在生产环境使用 `FastCommentsSSO::new_secure()` 并搭配您的 API key
2. **仅限服务器端**：在服务器上生成 SSO 令牌，切勿将您的 API key 暴露给客户端
3. **检查用户数据**：确保所有必需字段 (id, email, username) 已提供

### 异步运行时错误

该 SDK 使用 tokio 进行异步操作。请确保：

1. 将 tokio 添加到您的依赖项：
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. 使用 tokio 运行时：
```rust
#[tokio::main]
async fn main() {
    // 在此处放置您的异步代码
}
```