### Geauthenticeerde API's gebruiken (DefaultApi)

**Belangrijk:** Je moet je API-sleutel instellen op de ApiClient voordat je geauthenticeerde verzoeken doet. Als je dat niet doet, mislukken verzoeken met een 401-fout.

```ruby
require 'fastcomments-client'

# Maak en configureer de API-client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# VEREIST: Stel je API-sleutel in (haal deze uit je FastComments-dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Maak de API-instantie met de geconfigureerde client
api = FastCommentsClient::DefaultApi.new(api_client)

# Nu kun je geauthenticeerde API-aanroepen doen
begin
  # Voorbeeld: Voeg een SSO-gebruiker toe
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Veelvoorkomende fouten:
  # - 401: API-sleutel ontbreekt of is ongeldig
  # - 400: Verzoekvalidatie mislukt
end
```

### Gebruik van publieke API's (PublicApi)

Publieke eindpunten vereisen geen authenticatie:

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

### Veelvoorkomende problemen

1. **401 "missing-api-key" fout**: Zorg ervoor dat je `config.api_key['x-api-key'] = 'YOUR_KEY'` instelt voordat je de DefaultApi-instantie aanmaakt.
2. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server-side geauthenticeerde verzoeken, `PublicApi` voor client-side/publieke verzoeken.
3. **Null API-sleutel**: De SDK zal authenticatie stilzwijgend overslaan als de API-sleutel null is, wat leidt tot 401-fouten.