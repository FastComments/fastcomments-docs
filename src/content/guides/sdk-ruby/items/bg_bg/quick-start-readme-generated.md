### Използване на удостоверени API (DefaultApi)

**Важно:** Трябва да зададете вашия API ключ на ApiClient преди да правите удостоверени заявки. Ако не го направите, заявките ще се провалят с грешка 401.

```ruby
require 'fastcomments'

# Create and configure the API client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Create the API instance with the configured client
api = FastCommentsClient::DefaultApi.new(api_client)

# Now you can make authenticated API calls
begin
  # Example: Add an SSO user
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Common errors:
  # - 401: API key is missing or invalid
  # - 400: Request validation failed
end
```

### Използване на публични API (PublicApi)

Публичните крайни точки не изискват удостоверяване:

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

### Използване на Модерационни API (ModerationApi)

Методите за модерация захранват таблото на модератора. Предайте `sso` токен, за да бъде заявката изпратена от името на модератор, удостоверен чрез SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Example: List comments in the moderation queue
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Чести проблеми

1. **401 "missing-api-key" грешка**: Уверете се, че сте задали `config.api_key['x-api-key'] = 'YOUR_KEY'` преди да създадете инстанция на DefaultApi.
2. **Грешен API клас**: Използвайте `DefaultApi` за сървърни удостоверени заявки, `PublicApi` за клиентски/публични заявки и `ModerationApi` за заявки към таблото на модератора.
3. **Нулев API ключ**: SDK ще пропусне удостоверяването без да даде съобщение, ако API ключът е null, което води до грешки 401.