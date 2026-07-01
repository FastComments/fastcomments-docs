### Korzystanie z uwierzytelnionych API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w ApiClient przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

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

### Korzystanie z publicznych API (PublicApi)

Publiczne endpointy nie wymagają uwierzytelnienia:

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

### Korzystanie z API moderacji (ModerationApi)

Metody moderacji napędzają panel moderatora. Przekaż token `sso`, aby żądanie było wykonywane w imieniu moderatora uwierzytelnionego przez SSO:

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

### Typowe problemy

1. **Błąd 401 "missing-api-key"**: Upewnij się, że ustawiłeś `config.api_key['x-api-key'] = 'YOUR_KEY'` przed utworzeniem instancji DefaultApi.
2. **Nieprawidłowa klasa API**: Użyj `DefaultApi` dla serwerowych uwierzytelnionych żądań, `PublicApi` dla żądań po stronie klienta/publicznych oraz `ModerationApi` dla żądań panelu moderatora.
3. **Nullowy klucz API**: SDK cicho pominie uwierzytelnienie, jeśli klucz API jest null, co skutkuje błędami 401.