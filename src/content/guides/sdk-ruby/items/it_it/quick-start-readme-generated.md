### Utilizzo delle API Autenticate (DefaultApi)

**Importante:** Devi impostare la tua chiave API su ApiClient prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

```ruby
require 'fastcomments'

# Crea e configura il client API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBBLIGATORIO: Imposta la tua chiave API (prendila dalla dashboard di FastComments)
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
  # - 401: la chiave API manca o non è valida
  # - 400: la validazione della richiesta è fallita
end
```

### Utilizzo delle API Pubbliche (PublicApi)

Gli endpoint pubblici non richiedono autenticazione:

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

### Utilizzo delle API di Moderazione (ModerationApi)

I metodi di moderazione alimentano la dashboard dei moderatori. Fornisci un token `sso` in modo che la richiesta venga effettuata per conto di un moderatore autenticato tramite SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Esempio: Elenca i commenti nella coda di moderazione
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Problemi comuni

1. **401 "missing-api-key" error**: Assicurati di impostare `config.api_key['x-api-key'] = 'YOUR_KEY'` prima di creare l'istanza di DefaultApi.
2. **Classe API sbagliata**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste lato client/pubbliche e `ModerationApi` per le richieste della dashboard dei moderatori.
3. **Chiave API nulla**: L'SDK salterà silenziosamente l'autenticazione se la chiave API è nulla, causando errori 401.