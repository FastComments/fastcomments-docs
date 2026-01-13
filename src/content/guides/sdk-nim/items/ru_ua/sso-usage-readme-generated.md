### Простой SSO

```nim
import fastcomments/sso

let user = newSimpleSSOUserData(
  userId = "user-123",
  email = "user@example.com",
  avatar = "https://example.com/avatar.jpg"
)
let sso = newSimple(simpleUserData = user)
let token = sso.createToken()

echo "SSO Token: ", token
```

### Защищённый SSO

```nim
import fastcomments/sso

let user = newSecureSSOUserData(
  userId = "user-123",
  email = "user@example.com",
  username = "johndoe",
  avatar = "https://example.com/avatar.jpg"
)

let apiKey = "your-api-key"
let sso = newSecure(apiKey = apiKey, secureUserData = user)
let token = sso.createToken()

echo "Secure SSO Token: ", token
```