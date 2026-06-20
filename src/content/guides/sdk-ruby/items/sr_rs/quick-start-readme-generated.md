### Korišćenje autentifikованих API-ja (DefaultApi)

**Важно:** Морате подесити ваш API кључ на ApiClient пре него што пошаљете захтеве који захтевају аутентификацију. Ако то не урадите, захтеви ће пропасти са грешком 401.

```ruby
require 'fastcomments'

# Kreirajte i konfigurišite API klijenta
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Podesite vaš API ključ (preuzmite ga sa FastComments kontrolne table)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreirajte API instancu sa konfigurisanim klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete praviti autentifikovane API pozive
begin
  # Primer: Dodavanje SSO korisnika
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Uobičajene greške:
  # - 401: API ključ nedostaje ili je nevažeći
  # - 400: Validacija zahteva nije uspela
end
```

### Korišćenje јавних API-ja (PublicApi)

Јавни ендпоинти не захтевају аутентификацију:

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

### Korišćenje API-ja za модерацију (ModerationApi)

Методе за модерацију покрећу модераторски контролни панел. Проследите `sso` токен тако да се захтев изврши у име модератора који је аутентификован преко SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Пример: Прикaz komentara u redu za moderaciju
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Уобичајени проблеми

1. **401 "missing-api-key" error**: Уверите се да сте подесили `config.api_key['x-api-key'] = 'YOUR_KEY'` пре него што креирате DefaultApi инстанцу.
2. **Погрешна класа API-ја**: Користите `DefaultApi` за серверске захтеве са аутентификацијом, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве модераторског контролног панела.
3. **Null API key**: SDK ће ћутке прескочити аутентификацију ако је API кључ null, што доводи до 401 грешака.