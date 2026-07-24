### Kimlik Doğrulamalı API'leri Kullanma (DefaultApi)

**Önemli:** Kimlik doğrulamalı istekler yapmadan önce API anahtarınızı Configuration üzerinde ayarlamalısınız. Bunu yapmazsanız, istekler 401 hatasıyla başarısız olur.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API istemcisini oluştur ve yapılandır
config = Configuration()
config.host = "https://fastcomments.com"

# GEREKLİ: API anahtarınızı ayarlayın (bunu FastComments kontrol panelinizden alın)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Yapılandırılmış istemciyle API örneğini oluştur
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Artık kimlik doğrulamalı API çağrıları yapabilirsiniz
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

`ModerationApi`, moderatör panosunu çalıştırır. Yöntemler, bir `sso` token'ı geçirerek moderatör adına çağrılır:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Moderasyon bekleyen yorumları say
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (Tek Oturum Açma) Kullanımı

SDK, güvenli SSO token'ları oluşturmak için yardımcı programlar içerir:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Kullanıcı verilerini oluştur (id, email ve kullanıcı adı gereklidir)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Bunu API gizli anahtarınızla imzalayın (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Widget'a veya bir API çağrısına geçirmek için SSO token'ı oluştur
sso_token = sso.create_token()

# Bu token'ı ön uçta kullanın veya API çağrılarına geçirin
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

### Canlı Abonelikler (PubSub)

`pubsub` modülü, WebSocket üzerinden gerçek zamanlı yorum olaylarına (yeni yorumlar, oylar, düzenlemeler, bildirimler vb.) abone olmanızı sağlar ve FastComments Java SDK'sının `LiveEventSubscriber`ını yansıtır. Bu, oluşturulan API istemcisinin üzerine bir WebSocket istemcisi ekleyen `pubsub` ekstra paketini gerektirir:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # örn. bu oturum için bir UUID
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # EU bölgesi için "eu" olarak ayarlayın
)

# ...daha sonra, güncellemeleri artık istemediğinizde:
result.close()
```

Abone, bağlantıyı arka plan daemon iş parçacığında çalıştırır, jitter ile şeffaf bir şekilde yeniden bağlanır ve yeniden bağlanma sırasında event-log uç noktasından kaçırılan olayları alır. Kullanıcının göremeyeceği yorumların kimliklerini döndüren isteğe bağlı bir `can_see_comments` geri çağırma fonksiyonu (`List[str] -> Dict[str, str]`) sağlayarak, kullanıcının görüntüleyemeyeceği yorum olaylarını filtreleyebilirsiniz. `disable_live_commenting=True` ayarını yaparak `subscribe_to_changes` işlevini hiçbir şey yapmayan ve `None` döndüren bir işlev haline getirebilirsiniz.

### Yaygın Sorunlar

1. **401 "missing-api-key" hatası**: `DefaultApi` örneğini oluşturmadan önce `config.api_key = {"api_key": "YOUR_KEY"}` ayarladığınızdan emin olun.
2. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`, istemci tarafı/genel istekler için `PublicApi` ve moderatör panosu istekleri için `ModerationApi` kullanın.
3. **İçe aktarma hataları**: Doğru modülden içe aktardığınızdan emin olun:
   - API istemcisi: `from client import ...`
   - SSO yardımcı programları: `from sso import ...`
   - Canlı abonelikler: `from pubsub import ...` (`pubsub` ekstra paketi gerekir)