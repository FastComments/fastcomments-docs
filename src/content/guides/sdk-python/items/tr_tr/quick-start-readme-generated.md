### Kimlik Doğrulamalı API'leri Kullanma (DefaultApi)

**Important:** Kimlik doğrulamalı istekler yapmadan önce Configuration üzerinde API anahtarınızı ayarlamalısınız. Ayarlamazsanız, istekler 401 hatasıyla başarısız olur.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API istemcisini oluşturun ve yapılandırın
config = Configuration()
config.host = "https://fastcomments.com/api"

# GEREKLİ: API anahtarınızı ayarlayın (bunu FastComments panonuzdan alın)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Yapılandırılmış client ile API örneğini oluşturun
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Artık kimlik doğrulamalı API çağrıları yapabilirsiniz
try:
    # Örnek: Bir SSO kullanıcısı ekleyin
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Yaygın hatalar:
    # - 401: API anahtarı eksik veya geçersiz
    # - 400: İstek doğrulaması başarısız
```

### Herkese Açık API'leri Kullanma (PublicApi)

Herkese açık uç noktalar kimlik doğrulama gerektirmez:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Moderasyon Panosunu Kullanma (ModerationApi)

The `ModerationApi` moderatör panosunu sağlar. Yöntemler, bir moderatör adına bir `sso` token'i geçilerek çağrılır:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Moderasyonda bekleyen yorum sayısını alın
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (Single Sign-On) Kullanımı

SDK güvenli SSO tokenleri oluşturmak için yardımcı araçlar içerir:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kullanıcı verilerini oluşturun
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# API secret'inizle SSO örneği oluşturun
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# SSO token'ını oluşturun
sso_token = sso.create_token()

# Bu token'ı frontend'inizde kullanın veya API çağrılarına iletin
print(f"SSO Token: {sso_token}")
```

Basit SSO için (daha az güvenli, test amaçlı):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Yaygın Sorunlar

1. **401 "missing-api-key" hatası**: DefaultApi örneğini oluşturmadan önce `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` ayarladığınızdan emin olun.
2. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`, istemci/halka açık istekler için `PublicApi`, moderatör paneli istekleri için `ModerationApi` kullanın.
3. **İçe aktarma hataları**: Doğru modülden ithal ettiğinizden emin olun:
   - API istemcisi: `from client import ...`
   - SSO yardımcı araçları: `from sso import ...`