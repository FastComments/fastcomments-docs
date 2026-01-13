### Използване на удостоверени API (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ в ApiClient преди да правите удостоверени заявки. Ако не го направите, заявките ще се провалят с грешка 401.

```ruby
require 'fastcomments-client'

# Създаване и конфигуриране на API клиента
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ЗАДЪЛЖИТЕЛНО: Задайте вашия API ключ (вземете го от таблото на FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Създаване на екземпляр на API с конфигурирания клиент
api = FastCommentsClient::DefaultApi.new(api_client)

# Сега можете да правите удостоверени API повиквания
begin
  # Пример: Добавяне на SSO потребител
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Чести грешки:
  # - 401: Липсващ или невалиден API ключ
  # - 400: Провалена валидация на заявката
end
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват удостоверяване:

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

### Често срещани проблеми

1. **401 "missing-api-key" error**: Уверете се, че задавате `config.api_key['x-api-key'] = 'YOUR_KEY'` преди да създадете инстанция на DefaultApi.
2. **Wrong API class**: Използвайте `DefaultApi` за сървърни удостоверени заявки, `PublicApi` за клиентски/публични заявки.
3. **Null API key**: SDK ще пропусне удостоверяването без съобщение, ако API ключът е null, което ще доведе до грешки 401.