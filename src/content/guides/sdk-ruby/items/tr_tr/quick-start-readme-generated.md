### Using Authenticated APIs (DefaultApi)

**Önemli:** Kimliği doğrulanmış istekler yapmadan önce API anahtarını ApiClient üzerine ayarlamalısınız. Bunu yapmazsanız istekler 401 hatasıyla başarısız olur.

```ruby
require 'fastcomments'

# API istemcisini oluştur ve yapılandır
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# GEREKLİ: API anahtarınızı ayarlayın (FastComments kontrol panelinizden alın)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Yapılandırılmış istemciyle API örneğini oluştur
api = FastCommentsClient::DefaultApi.new(api_client)

# Artık kimliği doğrulanmış API çağrıları yapabilirsiniz
begin
  # Örnek: Bir SSO kullanıcısı ekle
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Yaygın hatalar:
  # - 401: API anahtarı eksik veya geçersiz
  # - 400: İstek doğrulaması başarısız oldu
end
```

### Using Public APIs (PublicApi)

Public uç noktalar kimlik doğrulama gerektirmez:

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    'YOUR_TENANT_ID',
    'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Using Moderation APIs (ModerationApi)

Moderasyon yöntemleri moderatör kontrol panelini güçlendirir. İsteğin bir SSO‑kimliği doğrulanmış moderatör adına yapılması için bir `sso` belirteci gönderin:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Örnek: Moderasyon kuyruğundaki yorumları listele
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Common Issues

1. **401 "missing-api-key" hatası**: `DefaultApi` örneğini oluşturmadan önce `config.api_key['x-api-key'] = 'YOUR_KEY'` ayarladığınızdan emin olun.
2. **Yanlış API sınıfı**: Sunucu tarafı kimliği doğrulanmış istekler için `DefaultApi`, istemci/halka açık istekler için `PublicApi` ve moderatör kontrol paneli istekleri için `ModerationApi` kullanın.
3. **Null API anahtarı**: SDK, API anahtarı null olduğunda kimlik doğrulamayı sessizce atlar ve bu da 401 hatalarına yol açar.