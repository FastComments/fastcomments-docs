### Използване на автентифицирани API (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ в ApiClient преди да правите автентифицирани заявки. Ако не го направите, заявките ще върнат грешка 401.

```ruby
require 'fastcomments'

# Създаване и конфигуриране на API клиента
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ЗАДЪЛЖИТЕЛНО: Задайте вашия API ключ (вземете го от таблото за FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Създайте инстанцията на API с конфигурирания клиент
api = FastCommentsClient::DefaultApi.new(api_client)

# Сега можете да правите автентифицирани API повиквания
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
  # - 401: API ключът липсва или е невалиден
  # - 400: Валидация на заявката неуспешна
end
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват автентикация:

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

### Използване на модерационни API (ModerationApi)

Методите за модерация захранват таблото за модератори. Предайте `sso` токен, така че заявката да се направи от името на модератор, автентифициран чрез SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Пример: Списък на коментарите в модерационната опашка
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Често срещани проблеми

1. **401 "missing-api-key" error**: Уверете се, че сте задали `config.api_key['x-api-key'] = 'YOUR_KEY'` преди да създадете инстанция на DefaultApi.
2. **Wrong API class**: Използвайте `DefaultApi` за автентифицирани заявки от страна на сървъра, `PublicApi` за клиентски/публични заявки и `ModerationApi` за заявки към таблото за модератори.
3. **Null API key**: SDK-то ще пропусне автентикацията без съобщение, ако API ключът е null, което ще доведе до грешки 401.