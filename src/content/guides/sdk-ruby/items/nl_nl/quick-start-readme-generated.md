### Gebruik geauthentificeerde API's (DefaultApi)

**Belangrijk:** U moet uw API‑sleutel instellen op de ApiClient voordat u geauthentificeerde verzoeken doet. Als u dat niet doet, zullen verzoeken falen met een 401‑fout.

```ruby
require 'fastcomments'

# Maak en configureer de API-client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# VERPLICHT: Stel uw API‑sleutel in (haal deze op van uw FastComments‑dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Maak de API‑instantie met de geconfigureerde client
api = FastCommentsClient::DefaultApi.new(api_client)

# Nu kunt u geauthentificeerde API‑aanroepen doen
begin
  # Voorbeeld: Voeg een SSO‑gebruiker toe
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
  # - 401: API‑sleutel ontbreekt of is ongeldig
  # - 400: Verzoekvalidatie mislukt
end
```

### Gebruik openbare API's (PublicApi)

Openbare eindpunten vereisen geen authenticatie:

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

### Gebruik moderatie‑API's (ModerationApi)

De moderatiemethoden voeden het moderator‑dashboard. Geef een `sso`‑token door zodat het verzoek wordt gedaan namens een SSO‑geauthentificeerde moderator:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Voorbeeld: Lijst opmerkingen in de moderatiewachtrij
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Veelvoorkomende problemen

1. **401 "missing-api-key" fout**: Zorg ervoor dat u `config.api_key['x-api-key'] = 'YOUR_KEY'` instelt vóór het maken van de DefaultApi‑instantie.  
2. **Verkeerde API‑klasse**: Gebruik `DefaultApi` voor server‑side geauthentificeerde verzoeken, `PublicApi` voor client‑side/openbare verzoeken, en `ModerationApi` voor verzoeken van het moderator‑dashboard.  
3. **Null API‑sleutel**: De SDK zal stilzwijgend authenticatie overslaan als de API‑sleutel null is, wat leidt tot 401‑fouten.