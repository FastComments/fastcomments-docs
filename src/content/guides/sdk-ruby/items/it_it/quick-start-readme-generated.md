### Utilizzo delle API Autenticate (DefaultApi)

**Importante:** Devi impostare la tua chiave API su ApiClient prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

```ruby
require 'fastcomments-client'

# Crea e configura il client API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBBLIGATORIO: Imposta la tua chiave API (prendila dalla tua dashboard FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Crea l'istanza API con il client configurato
api = FastCommentsClient::DefaultApi.new(api_client)

# Ora puoi effettuare chiamate API autenticate
begin
  # Esempio: Aggiungi un utente SSO
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Errori comuni:
  # - 401: la chiave API è mancante o non valida
  # - 400: la validazione della richiesta è fallita
end
```

### Utilizzo delle API Pubbliche (PublicApi)

Gli endpoint pubblici non richiedono l'autenticazione:

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

### Problemi comuni

1. **Errore 401 "missing-api-key"**: Assicurati di impostare `config.api_key['x-api-key'] = 'YOUR_KEY'` prima di creare l'istanza DefaultApi.
2. **Classe API errata**: Usa `DefaultApi` per le richieste lato server autenticate, `PublicApi` per le richieste lato client/pubbliche.
3. **Chiave API nulla**: Lo SDK salterà silenziosamente l'autenticazione se la chiave API è null, causando errori 401.