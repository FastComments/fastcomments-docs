### Использование аутентифицированных API (DefaultApi)

**Важно:** Вы должны установить ваш API-ключ в ApiClient перед выполнением аутентифицированных запросов. Если не сделаете, запросы вернут ошибку 401.

```ruby
require 'fastcomments'

# Создать и настроить клиент API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ОБЯЗАТЕЛЬНО: Установите ваш API-ключ (получите его в панели FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Создать экземпляр API с настроенным клиентом
api = FastCommentsClient::DefaultApi.new(api_client)

# Теперь вы можете делать аутентифицированные вызовы API
begin
  # Пример: Добавить SSO пользователя
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Общие ошибки:
  # - 401: API-ключ отсутствует или недействителен
  # - 400: Ошибка проверки запроса
end
```

### Использование публичных API (PublicApi)

Публичные эндпоинты не требуют аутентификации:

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

### Использование API модерации (ModerationApi)

Методы модерации обеспечивают работу панели модератора. Передайте токен `sso`, чтобы запрос выполнялся от имени модератора, аутентифицированного через SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Пример: Получить список комментариев в очереди модерации
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Распространённые проблемы

1. **Ошибка 401 "missing-api-key"**: Убедитесь, что вы установили `config.api_key['x-api-key'] = 'YOUR_KEY'` перед созданием экземпляра DefaultApi.
2. **Неправильный класс API**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов и `ModerationApi` для запросов панели модератора.
3. **Null API-ключ**: SDK будет молча пропускать аутентификацию, если API-ключ равен null, что приведет к ошибкам 401.