### Yetkilendirilmiş API'leri Kullanma (DefaultApi)

**Önemli:** Yetkilendirilmiş istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamanız gerekir. Ayarlamazsanız istekler 401 hatası ile başarısız olur.

```ruby
require 'fastcomments'

# API istemcisini oluşturun ve yapılandırın
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# GEREKLİ: API anahtarınızı ayarlayın (bunu FastComments kontrol panelinizden alın)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Yapılandırılmış istemci ile API örneğini oluşturun
api = FastCommentsClient::DefaultApi.new(api_client)

# Artık yetkilendirilmiş API çağrıları yapabilirsiniz
begin
  # Örnek: Bir SSO kullanıcısı ekleyin
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

### Public API'leri Kullanma (PublicApi)

Genel uç noktalar kimlik doğrulama gerektirmez:

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    tenant_id: 'YOUR_TENANT_ID',
    url_id: 'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Moderasyon API'lerini Kullanma (ModerationApi)

Moderasyon yöntemleri moderatör panosunu besler. İsteğin bir SSO ile kimlik doğrulanmış moderatör adına yapılması için bir `sso` tokeni geçirin:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Örnek: Moderasyon kuyruğundaki yorumları listeleyin
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Yaygın Sorunlar

1. **401 "missing-api-key" hatası**: DefaultApi örneğini oluşturmadan önce `config.api_key['x-api-key'] = 'YOUR_KEY'` ayarladığınızdan emin olun.
2. **Yanlış API sınıfı**: Sunucu tarafı yetkilendirilmiş istekler için `DefaultApi`, istemci tarafı/genel istekler için `PublicApi`, ve moderatör panosu istekleri için `ModerationApi` kullanın.
3. **Null API anahtarı**: Eğer API anahtarı null ise SDK kimlik doğrulamayı sessizce atlar ve bu 401 hatalarına yol açar.