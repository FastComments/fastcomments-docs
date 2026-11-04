### Kimlik Doğrulamalı API'leri Kullanma (DefaultApi)

**Önemli:** Kimlik doğrulamalı istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamanız gerekir. Ayarlamazsanız, istekler 401 hatasıyla başarısız olur.

```ruby
require 'fastcomments'

# API istemcisini oluşturun ve yapılandırın
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# GEREKLİ: API anahtarınızı ayarlayın (bunu FastComments panonuzdan alın)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Yapılandırılmış istemciyle API örneğini oluşturun
api = FastCommentsClient::DefaultApi.new(api_client)

# Artık kimlik doğrulamalı API çağrıları yapabilirsiniz
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

### Genel API'leri Kullanma (PublicApi)

Herkese açık uç noktalar kimlik doğrulaması gerektirmez:

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

### Moderasyon API'lerini Kullanma (ModerationApi)

Moderasyon metotları moderatör panelini çalıştırır. İsteğin SSO ile kimlik doğrulamış bir moderatör adına yapılması için bir `sso` token'ı geçirin:

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
2. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`, istemci tarafı/herkese açık istekler için `PublicApi` ve moderatör paneli istekleri için `ModerationApi` kullanın.
3. **Null API anahtarı**: API anahtarı null ise SDK kimlik doğrulamasını sessizce atlar ve bu da 401 hatalarına yol açar.