### Brug af autentificerede API'er (DefaultApi)

**Vigtigt:** Du skal angive din API-nøgle på ApiClient, før du foretager autentificerede anmodninger. Hvis du ikke gør det, vil anmodninger fejle med en 401-fejl.

```ruby
require 'fastcomments'

# Opret og konfigurer API-klienten
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# KRÆVET: Angiv din API-nøgle (hent den fra dit FastComments-dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Opret API-instansen med den konfigurerede klient
api = FastCommentsClient::DefaultApi.new(api_client)

# Nu kan du lave autentificerede API-kald
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

Offentlige endepunkter kræver ikke autentificering:

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

### Brug af moderation-API'er (ModerationApi)

Moderationsmetoderne driver moderator-dashboardet. Send et `sso`-token, så anmodningen foretages på vegne af en SSO-autentificeret moderator:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Eksempel: Hent kommentarer i moderator-køen
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Almindelige problemer

1. **401 "missing-api-key" fejl**: Sørg for at du sætter `config.api_key['x-api-key'] = 'YOUR_KEY'` før du opretter DefaultApi-instansen.
2. **Forkert API-klasse**: Brug `DefaultApi` til autentificerede anmodninger på serversiden, `PublicApi` til anmodninger på klientsiden/offentlige anmodninger, og `ModerationApi` til anmodninger fra moderator-dashboardet.
3. **Null API key**: SDK'en vil stiltiende springe autentificering over, hvis API-nøglen er null, hvilket fører til 401-fejl.