### Коришћење аутентификованих API‑ја (DefaultApi)

**Важно:** Морате поставити ваш API кључ у ApiClient пре него што направите аутентификоване захтеве. Ако то не урадите, захтеви ће пропасти са грешком 401.

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

### Коришћење јавних API‑ја (PublicApi)

Јавни крајњи тачке не захтевају аутентификацију:

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

### Коришћење модерацијских API‑ја (ModerationApi)

Методе за модерацију покрећу контролни панел модератора. Проследите `sso` токен тако да се захтев изврши у име модератора аутентификованог преко SSO:

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

### Уобичајени проблеми

1. **401 „missing-api-key“ грешка**: Уверите се да сте поставили `config.api_key['x-api-key'] = 'YOUR_KEY'` пре креирања DefaultApi инстанце.
2. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве контролног панела модератора.
3. **Нулти API кључ**: SDK ће тихо прескочити аутентификацију ако је API кључ нулти, што доводи до грешака 401.