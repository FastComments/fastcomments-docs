### Korištenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ na ApiClient prije nego što napravite autentifikovane zahtjeve. Ako to ne učinite, zahtjevi će završiti sa 401 greškom.

```ruby
require 'fastcomments-client'

# Kreirajte i konfigurirajte API klijenta
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Postavite vaš API ključ (preuzmite ga sa FastComments kontrolne ploče)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreirajte instancu API-ja sa konfiguriranim klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete napraviti autentifikovane API pozive
begin
  # Primjer: Dodavanje SSO korisnika
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
  # - 401: API ključ nedostaje ili nije važeći
  # - 400: Validacija zahtjeva nije uspjela
end
```

### Korištenje javnih API-ja (PublicApi)

Javni endpointi ne zahtijevaju autentifikaciju:

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

1. **401 "missing-api-key" greška**: Provjerite da li ste postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` prije kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za zahtjeve na serverskoj strani koji zahtijevaju autentifikaciju, `PublicApi` za klijentske/javne zahtjeve.
3. **Null API ključ**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što će dovesti do 401 grešaka.