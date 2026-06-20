### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате поставити свој API кључ у ApiClient пре слања аутентификованих захтева. Ако то не урадите, захтеви ће се завршити грешком 401.

```ruby
require 'fastcomments'

# Креирајте и конфигуришите API клијент
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ОБАВЕЗНО: Поставите свој API кључ (добијете га у својој FastComments контролној табли)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Креирајте пример API са конфигурисаним клијентом
api = FastCommentsClient::DefaultApi.new(api_client)

# Сада можете правити аутентификоване API позиве
begin
  # Пример: Додајте SSO корисника
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Уобичајене грешке:
  # - 401: API кључ недостаје или је неважећи
  # - 400: Валидација захтева није успела
end
```

### Коришћење јавних API-ја (PublicApi)

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

### Коришћење модерацијских API-ја (ModerationApi)

Методе за модерацију покрећу контролну таблу модератора. Проследите `sso` токен тако да се захтев изврши у име модератора који је аутентификован преко SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Пример: Листа коментара у модерацијској реди
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Чести проблеми

1. **401 "missing-api-key" error**: Побрините се да поставите `config.api_key['x-api-key'] = 'YOUR_KEY'` пре него што креирате инстанцу DefaultApi.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве контролне табле модератора.
3. **Null API key**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што ће довести до 401 грешака.