### 401 Неовлашћене грешке

If you're getting 401 errors when using the authenticated API:

1. **Provjerite svoj API ključ**: Ensure you're using the correct API key from your FastComments dashboard
2. **Provjerite tenant ID**: Make sure the tenant ID matches your account
3. **Format API ključa**: The API key should be set as the `x-api-key` header on the shared configuration:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Korištenje pogrešnog API-ja**: Make sure you're using `DefaultAPI` (not `PublicAPI`) for authenticated calls

### Problemi sa SSO tokenom

If SSO tokens aren't working:

1. **Koristite sigurni način za produkciju**: Always use `FastCommentsSSO.createSecure()` with your API key for production
2. **Samo na serveru**: Generate secure SSO tokens on your server, never expose your API key to clients
3. **Provjerite korisničke podatke**: Ensure all required fields (id, email, username) are provided
4. **Istek tokena**: Secure SSO tokens include a timestamp and may expire. Generate fresh tokens as needed.

### SSL/TLS greške

If you encounter SSL/TLS errors:

1. Ensure your app's Info.plist allows HTTPS connections to fastcomments.com
2. Check that you're not using App Transport Security exceptions that might block the connection