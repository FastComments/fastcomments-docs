### Verwendung authentifizierter APIs (DefaultApi)

**Wichtig:** Sie müssen Ihren API-Schlüssel im ApiClient festlegen, bevor Sie authentifizierte Anfragen stellen. Wenn Sie dies nicht tun, schlagen die Anfragen mit einem 401-Fehler fehl.

```ruby
require 'fastcomments'

# Erstelle und konfiguriere den API-Client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ERFORDERLICH: Setzen Sie Ihren API-Schlüssel (holen Sie ihn von Ihrem FastComments-Dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Erstelle die API-Instanz mit dem konfigurierten Client
api = FastCommentsClient::DefaultApi.new(api_client)

# Jetzt können Sie authentifizierte API-Aufrufe tätigen
begin
  # Beispiel: Einen SSO-Benutzer hinzufügen
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Häufige Fehler:
  # - 401: API-Schlüssel fehlt oder ist ungültig
  # - 400: Anfragevalidierung fehlgeschlagen
end
```

### Verwendung öffentlicher APIs (PublicApi)

Öffentliche Endpunkte erfordern keine Authentifizierung:

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

### Verwendung von Moderations-APIs (ModerationApi)

Die Moderationsmethoden betreiben das Moderator‑Dashboard. Übergeben Sie ein `sso`‑Token, damit die Anfrage im Namen eines SSO‑authentifizierten Moderators gestellt wird:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Beispiel: Kommentare in der Moderationswarteschlange auflisten
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Häufige Probleme

1. **401 "missing-api-key" Fehler**: Stellen Sie sicher, dass Sie `config.api_key['x-api-key'] = 'YOUR_KEY'` setzen, bevor Sie die DefaultApi-Instanz erstellen.  
2. **Falsche API-Klasse**: Verwenden Sie `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige/öffentliche Anfragen und `ModerationApi` für Anfragen des Moderator‑Dashboards.  
3. **Null API-Schlüssel**: Das SDK wird die Authentifizierung stillschweigend überspringen, wenn der API-Schlüssel null ist, was zu 401-Fehlern führt.