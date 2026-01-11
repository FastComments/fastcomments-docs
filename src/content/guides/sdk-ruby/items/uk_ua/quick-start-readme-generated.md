### Використання автентифікованих API (DefaultApi)

**Важливо:** Ви повинні встановити свій API-ключ на ApiClient перед виконанням автентифікованих запитів. Якщо цього не зробити, запити повернуть помилку 401.

```ruby
require 'fastcomments-client'

# Створіть і налаштуйте клієнт API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ОБОВ'ЯЗКОВО: Встановіть свій API-ключ (отримайте його на панелі FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Створіть екземпляр API з налаштованим клієнтом
api = FastCommentsClient::DefaultApi.new(api_client)

# Тепер ви можете виконувати автентифіковані виклики API
begin
  # Приклад: Додати SSO-користувача
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Поширені помилки:
  # - 401: API-ключ відсутній або недійсний
  # - 400: Помилка валідації запиту
end
```

### Використання публічних API (PublicApi)

Публічні кінцеві точки не вимагають автентифікації:

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

### Поширені проблеми

1. **401 "missing-api-key" error**: Переконайтеся, що ви встановили `config.api_key['x-api-key'] = 'YOUR_KEY'` перед створенням екземпляра DefaultApi.
2. **Неправильний клас API**: Використовуйте `DefaultApi` для серверних автентифікованих запитів, `PublicApi` для клієнтських/публічних запитів.
3. **Null API key**: SDK тихо пропустить автентифікацію, якщо API-ключ дорівнює null, що призведе до помилок 401.