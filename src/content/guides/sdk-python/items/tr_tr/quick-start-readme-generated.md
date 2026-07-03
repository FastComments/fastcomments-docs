### Kimlik Doğrulamalı API'leri Kullanma (DefaultApi)

**Önemli:** Kimlik doğrulamalı isteklerde bulunmadan önce API anahtarınızı Configuration üzerine ayarlamalısınız. Aksi takdirde istekler 401 hatası ile başarısız olur.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API istemcisini oluştur ve yapılandır
config = Configuration()
config.host = "https://fastcomments.com"

# GEREKLİ: API anahtarınızı ayarlayın (FastComments kontrol panelinizden alın)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Yapılandırılmış istemciyle API örneğini oluştur
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Artık kimlik doğrulamalı API çağrıları yapabilirsiniz
try:
    # Örnek: Bir SSO kullanıcısı ekleme
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Yaygın hatalar:
    # - 401: API anahtarı eksik veya geçersiz
    # - 400: İstek doğrulaması başarısız
```

### Genel API'leri Kullanma (PublicApi)

Genel uç noktalar kimlik doğrulama gerektirmez:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Moderasyon Panosunu Kullanma (ModerationApi)

`ModerationApi`, moderatör panosunu güçlendirir. Yöntemler, bir `sso` jetonu geçilerek moderatör adına çağrılır:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Moderasyonda bekleyen yorumları sayma
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (Tek Kullanımlı Oturum Açma) Kullanma

SDK, güvenli SSO jetonları oluşturmak için yardımcı araçlar içerir:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kullanıcı verilerini oluştur (id, email ve username zorunludur)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# API gizli anahtarınızla imzalayın (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Widget'a veya API çağrısına geçirilecek SSO jetonunu oluştur
sso_token = sso.create_token()

# Bu jetonu ön uçta kullanın veya API çağrılarına geçirin
print(f"SSO Token: {sso_token}")
```

Basit SSO için (daha az güvenli, test amaçlı):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Yaygın Sorunlar

1. **401 "missing-api-key" hatası**: `DefaultApi` örneğini oluşturmadan önce `config.api_key = {"api_key": "YOUR_KEY"}` ayarladığınızdan emin olun.
2. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`, istemci/öffentlich istekler için `PublicApi` ve moderatör paneli istekleri için `ModerationApi` kullanın.
3. **İçe aktarma hataları**: Doğru modülden ithal ettiğinizden emin olun:
   - API istemcisi: `from client import ...`
   - SSO yardımcıları: `from sso import ...`