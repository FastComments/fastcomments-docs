### Использование аутентифицированных API (DefaultApi)

**Важно:** Вы должны установить ваш API-ключ в ApiClient перед выполнением аутентифицированных запросов. Если вы этого не сделаете, запросы завершатся с ошибкой 401.

```ruby
require 'fastcomments'

# Создайте и настройте API-клиент
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ОБЯЗАТЕЛЬНО: Установите ваш API-ключ (получите его на панели управления FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Создайте экземпляр API с настроенным клиентом
api = FastCommentsClient::DefaultApi.new(api_client)

# Теперь вы можете выполнять аутентифицированные вызовы API
begin
  # Пример: Добавить SSO-пользователя
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
  # - 400: Ошибка валидации запроса
end
```

### Использование публичных API (PublicApi)

Публичные конечные точки не требуют аутентификации:

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

### Использование API модерации (ModerationApi)

Методы модерации обеспечивают работу панели модератора. Передайте токен `sso`, чтобы запрос выполнялся от имени модератора, аутентифицированного через SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Пример: Список комментариев в очереди модерации
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Распространённые проблемы

1. **401 "missing-api-key" error**: Убедитесь, что вы установили `config.api_key['x-api-key'] = 'YOUR_KEY'` перед созданием экземпляра DefaultApi.
2. **Wrong API class**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов и `ModerationApi` для запросов панели модератора.
3. **Null API key**: SDK тихо пропустит аутентификацию, если API-ключ равен null, что приведёт к ошибкам 401.