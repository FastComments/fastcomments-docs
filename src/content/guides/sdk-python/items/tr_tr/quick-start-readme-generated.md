### Yetkilendirilmiş API'leri Kullanma (DefaultApi)

**Önemli:** Yetkilendirilmiş istekler yapmadan önce API anahtarınızı Configuration üzerinde ayarlamalısınız. Aksi takdirde istekler 401 hatasıyla başarısız olacaktır.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API istemcisini oluştur ve yapılandır
config = Configuration()
config.host = "https://fastcomments.com/api"

# GEREKLİ: API anahtarınızı ayarlayın (FastComments kontrol panelinizden alın)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Yapılandırılmış istemciyle API örneğini oluştur
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Artık yetkilendirilmiş API çağrıları yapabilirsiniz
try:
    # Örnek: Bir SSO kullanıcısı ekle
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
    # - 400: İstek doğrulama başarısız
```

### Genel API'leri Kullanma (PublicApi)

Genel uç noktalar kimlik doğrulama gerektirmez:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Moderasyon Panosunu Kullanma (ModerationApi)

`ModerationApi`, moderatör panosunun gücünü sağlar. Yöntemler bir `sso` belirteci geçerek moderatör adına çağrılır:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Moderasyon bekleyen yorumları say
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (Tekli Oturum Açma) Kullanma

SDK, güvenli SSO belirteçleri oluşturmak için yardımcı programlar içerir:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kullanıcı verisi oluştur
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# API gizlinizle SSO örneği oluştur
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# SSO belirtecini oluştur
sso_token = sso.create_token()

# Bu belirteci ön yüzünüzde kullanın veya API çağrılarına geçirin
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
2. **Yanlış API sınıfı**: Sunucu tarafı yetkilendirilmiş istekler için `DefaultApi`, istemci tarafı/genel istekler için `PublicApi` ve moderatör panosu istekleri için `ModerationApi` kullanın.
3. **İçe aktarma hataları**: Doğru modülden ithal ettiğinizden emin olun:
   - API istemcisi: `from client import ...`
   - SSO yardımcı programları: `from sso import ...`