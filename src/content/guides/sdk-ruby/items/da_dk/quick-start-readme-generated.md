### Brugen af godkendte API'er (DefaultApi)

**Vigtigt:** Du skal sætte din API-nøgle på ApiClient, før du laver godkendte anmodninger. Hvis du ikke gør det, vil anmodninger mislykkes med en 401-fejl.

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

### Brugen af offentlige API'er (PublicApi)

Offentlige slutpunkter kræver ikke godkendelse:

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

### Brugen af moderations-API'er (ModerationApi)

Moderationsmetoderne driver moderator‑dashboardet. Overgiv en `sso`‑token, så anmodningen foretages på vegne af en SSO-godkendt moderator:

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

### Almindelige problemer

1. **401 "missing-api-key" fejl**: Sørg for, at du sætter `config.api_key['x-api-key'] = 'YOUR_KEY'` før du opretter DefaultApi‑instansen.
2. **Forkert API‑klasse**: Brug `DefaultApi` til server‑side godkendte anmodninger, `PublicApi` til klient‑side/offentlige anmodninger, og `ModerationApi` til anmodninger fra moderator‑dashboardet.
3. **Null API‑nøgle**: SDK'en vil stille og roligt springe godkendelse over, hvis API‑nøglen er null, hvilket medfører 401‑fejl.