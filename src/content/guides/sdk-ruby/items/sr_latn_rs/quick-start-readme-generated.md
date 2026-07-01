### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ na ApiClient pre slanja autentifikovanih zahteva. Ako to ne učinite, zahtevi će propasti sa greškom 401.

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

### Korišćenje javnih API-ja (PublicApi)

Javni endpointi ne zahtevaju autentifikaciju:

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

### Korišćenje moderacionih API-ja (ModerationApi)

Metode za moderaciju napajaju moderatorsku kontrolnu tablu. Prosledite `sso` token kako bi zahtev bio izvršen u ime SSO‑autentifikovanog moderatora:

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

### Uobičajeni problemi

1. **401 "missing-api-key" greška**: Uverite se da ste postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` pre kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve i `ModerationApi` za zahteve moderatorske kontrolne table.
3. **Null API ključ**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što dovodi do 401 grešaka.