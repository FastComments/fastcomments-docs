### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате поставити ваш API кључ на ApiClient пре прављења аутентификованих захтева. Ако то не урадите, захтеви ће пропасти са грешком 401.

```ruby
require 'fastcomments-client'

# Креирајте и подесите API клијента
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ОБАВЕЗНО: Поставите ваш API кључ (преузмите га са FastComments контролне табле)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Креирајте инстанцу API-ја са конфигурисаним клијентом
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
require 'fastcomments-client'

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

### Уобичајени проблеми

1. **401 "missing-api-key" грешка**: Уверите се да сте поставили `config.api_key['x-api-key'] = 'YOUR_KEY'` пре креирања инстанце DefaultApi.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве.
3. **Null API кључ**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што доводи до 401 грешака.