### Коришћење аутентификованих API-ја (DefaultApi)

**Важно:** Морате подесити ваш API кључ на ApiClient пре слања аутентификованих захтјева. Ако то не урадите, захтјеви ће пропасти са 401 грешком.

```ruby
require 'fastcomments-client'

# Креирајте и конфигуришите API клијента
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ПОТРЕБНО: Подесите ваш API кључ (преузмите га из FastComments контролне табле)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Направите инстанцу API-ја са конфигурисаним клијентом
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
  # Честе грешке:
  # - 401: API кључ недостаје или је неважећи
  # - 400: Валидација захтјева није успјела
end
```

### Коришћење јавних API-ја (PublicApi)

Јавне крајње тачке не захтијевају аутентификацију:

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

### Чести проблеми

1. **401 "missing-api-key" грешка**: Обавезно подесите `config.api_key['x-api-key'] = 'YOUR_KEY'` прије креирања DefaultApi инстанце.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтјеве, `PublicApi` за клијентске/јавне захтјеве.
3. **Null API кључ**: SDK ће тихо прескочити аутентификацију ако је API кључ null, што ће довести до 401 грешака.