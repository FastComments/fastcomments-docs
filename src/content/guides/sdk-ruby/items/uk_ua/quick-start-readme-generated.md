### Використання автентифікованих API (DefaultApi)

**Important:** Ви повинні встановити свій API-ключ у ApiClient перед виконанням автентифікованих запитів. Якщо цього не зробити, запити повернуть помилку 401.

```ruby
require 'fastcomments'

# Створити та налаштувати клієнт API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ОБОВ'ЯЗКОВО: Встановіть свій API-ключ (отримайте його на панелі керування FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Створити екземпляр API з налаштованим клієнтом
api = FastCommentsClient::DefaultApi.new(api_client)

# Тепер ви можете робити автентифіковані виклики API
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
  # - 400: Перевірка запиту не пройшла
end
```

### Використання публічних API (PublicApi)

Публічні кінцеві точки не вимагають автентифікації:

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

### Використання модераційних API (ModerationApi)

Методи модерації забезпечують роботу панелі модератора. Передайте токен `sso`, щоб запит був виконаний від імені модератора, автентифікованого через SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Приклад: Список коментарів у черзі модерації
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Поширені проблеми

1. **401 "missing-api-key" error**: Переконайтеся, що ви встановили `config.api_key['x-api-key'] = 'YOUR_KEY'` перед створенням екземпляра DefaultApi.
2. **Wrong API class**: Використовуйте `DefaultApi` для автентифікованих запитів на стороні сервера, `PublicApi` для клієнтських/публічних запитів та `ModerationApi` для запитів панелі модератора.
3. **Null API key**: SDK тихо пропустить автентифікацію, якщо API-ключ дорівнює null, що призведе до помилок 401.