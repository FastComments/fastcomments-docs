### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate da postavite svoj API ključ na ApiClient pre nego što izvršite autentifikovane zahteve. Ako to ne učinite, zahtevi će završiti sa greškom 401.

```ruby
require 'fastcomments-client'

# Kreiraj i konfiguriši API klijent
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Postavite svoj API ključ (nabavite ga na vašem FastComments kontrolnom panelu)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreirajte instancu API-ja sa konfigurisanim klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete izvršavati autentifikovane API pozive
begin
  # Primer: Dodavanje SSO korisnika
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Uobičajene greške:
  # - 401: API ključ nedostaje ili je nevažeći
  # - 400: Validacija zahteva nije uspela
end
```

### Korišćenje javnih API-ja (PublicApi)

Javni endpointi ne zahtevaju autentifikaciju:

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

### Uobičajeni problemi

1. **401 "missing-api-key" error**: Uverite se da ste postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` pre nego što kreirate instancu DefaultApi.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za autentifikovane zahteve sa serverske strane, a `PublicApi` za zahteve sa klijentske strane/javne zahteve.
3. **Null API ključ**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što dovodi do 401 grešaka.