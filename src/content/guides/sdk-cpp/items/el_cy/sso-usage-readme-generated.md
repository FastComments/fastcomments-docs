---
### Απλό SSO

```cpp
#include <fastcomments/sso/fastcomments_sso.hpp>
#include <iostream>

using namespace fastcomments::sso;

int main() {
    SimpleSSOUserData user("user-123", "user@example.com", "https://example.com/avatar.jpg");
    FastCommentsSSO sso = FastCommentsSSO::newSimple(user);
    std::string token = sso.createToken();

    std::cout << "SSO Token: " << token << std::endl;
    return 0;
}
```

### Ασφαλές SSO

```cpp
#include <fastcomments/sso/fastcomments_sso.hpp>
#include <iostream>

using namespace fastcomments::sso;

int main() {
    SecureSSOUserData user("user-123", "user@example.com", "johndoe", "https://example.com/avatar.jpg");

    std::string apiKey = "your-api-key";
    FastCommentsSSO sso = FastCommentsSSO::newSecure(apiKey, user);
    std::string token = sso.createToken();

    std::cout << "Secure SSO Token: " << token << std::endl;
    return 0;
}
```
---