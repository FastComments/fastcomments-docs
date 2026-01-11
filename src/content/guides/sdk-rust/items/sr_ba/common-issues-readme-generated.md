### 401 Unauthorized Errors

If you're getting 401 errors when using the authenticated API:

1. **Check your API key**: Ensure you're using the correct API key from your FastComments dashboard
2. **Verify the tenant ID**: Make sure the tenant ID matches your account
3. **API key format**: The API key should be passed in the Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO Token Issues

If SSO tokens aren't working:

1. **Use secure mode for production**: Always use `FastCommentsSSO::new_secure()` with your API key for production
2. **Server-side only**: Generate SSO tokens on your server, never expose your API key to clients
3. **Check user data**: Ensure all required fields (id, email, username) are provided

### Async Runtime Errors

The SDK uses tokio for async operations. Make sure to:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Ваш асинхрони код овдје
}
```