### Utilizzo delle API Autenticate (DefaultApi)

**Importante:** È necessario impostare la chiave API sul ApiClient prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

```ruby
require 'fastcomments'

# Crea e configura il client API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBBLIGATORIO: Imposta la tua chiave API (ottienila dalla tua dashboard FastComments)
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
  # - 401: Chiave API mancante o non valida
  # - 400: Convalida della richiesta non riuscita
end
```

### Utilizzo delle API Pubbliche (PublicApi)

Gli endpoint pubblici non richiedono autenticazione:

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

### Utilizzo delle API di Moderazione (ModerationApi)

I metodi di moderazione alimentano la dashboard del moderatore. Passa un token `sso` in modo che la richiesta sia effettuata per conto di un moderatore autenticato via SSO:

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

### Problemi Comuni

1. **Errore 401 "missing-api-key"**: Assicurati di impostare `config.api_key['x-api-key'] = 'YOUR_KEY'` prima di creare l'istanza DefaultApi.
2. **Classe API errata**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste lato client/pubbliche, e `ModerationApi` per richieste della dashboard del moderatore.
3. **Chiave API nulla**: L'SDK ignorerà silenziosamente l'autenticazione se la chiave API è nulla, portando a errori 401.