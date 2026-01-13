### Brug af godkendte API'er (DefaultApi)

**Vigtigt:** Du skal sætte din API-nøgle på ApiClient, inden du foretager godkendte anmodninger. Hvis du ikke gør det, vil anmodninger mislykkes med en 401-fejl.

```ruby
require 'fastcomments-client'

# Opret og konfigurer API-klienten
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# KRÆVET: Angiv din API-nøgle (find den på dit FastComments-dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Opret API-forekomsten med den konfigurerede klient
api = FastCommentsClient::DefaultApi.new(api_client)

# Nu kan du foretage godkendte API-opkald
begin
  # Eksempel: Tilføj en SSO-bruger
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Almindelige fejl:
  # - 401: API-nøglen mangler eller er ugyldig
  # - 400: Anmodningsvalidering mislykkedes
end
```

### Brug af offentlige API'er (PublicApi)

Offentlige endpoints kræver ikke godkendelse:

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

### Almindelige problemer

1. **401 "missing-api-key" fejl**: Sørg for at sætte `config.api_key['x-api-key'] = 'YOUR_KEY'` før du opretter DefaultApi-instansen.
2. **Forkert API-klasse**: Brug `DefaultApi` for server-side godkendte anmodninger, `PublicApi` for klient-side/offentlige anmodninger.
3. **Null API-nøgle**: SDK'en vil stiltiende springe autentificering over, hvis API-nøglen er null, hvilket fører til 401-fejl.