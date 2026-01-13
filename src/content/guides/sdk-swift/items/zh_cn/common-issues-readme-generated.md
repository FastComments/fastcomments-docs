### 401 Unauthorized Errors

If you're getting 401 errors when using the authenticated API:

1. **Check your API key**: Ensure you're using the correct API key from your FastComments dashboard
2. **Verify the tenant ID**: Make sure the tenant ID matches your account
3. **API key format**: The API key should be set on the API client:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Using the wrong API**: Make sure you're using `DefaultAPI` (not `PublicAPI`) for authenticated calls

### SSO Token Issues

If SSO tokens aren't working:

1. **Use secure mode for production**: Always use `FastCommentsSSO.createSecure()` with your API key for production
2. **Server-side only**: Generate secure SSO tokens on your server, never expose your API key to clients
3. **Check user data**: Ensure all required fields (id, email, username) are provided
4. **Token expiration**: Secure SSO tokens include a timestamp and may expire. Generate fresh tokens as needed.

### SSL/TLS Errors

If you encounter SSL/TLS errors:

1. Ensure your app's Info.plist allows HTTPS connections to fastcomments.com
2. Check that you're not using App Transport Security exceptions that might block the connection