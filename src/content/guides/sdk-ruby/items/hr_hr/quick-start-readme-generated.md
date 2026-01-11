### Korištenje autentificiranih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ u ApiClient prije slanja autentificiranih zahtjeva. Ako to ne učinite, zahtjevi će vratiti pogrešku 401.

```ruby
require 'fastcomments-client'

# Kreirajte i konfigurirajte API klijent
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Postavite svoj API ključ (dohvatite ga s vaše FastComments nadzorne ploče)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreirajte instancu API-ja s konfiguriranim klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete izvršavati autentificirane API pozive
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
  # Uobičajene pogreške:
  # - 401: API ključ nedostaje ili nije valjan
  # - 400: Provjera valjanosti zahtjeva nije uspjela
end
```

### Korištenje javnih API-ja (PublicApi)

Javne krajnje točke ne zahtijevaju autentifikaciju:

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

1. **401 "missing-api-key" pogreška**: Provjerite jeste li postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` prije nego što stvorite instancu DefaultApi.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za serverske autentificirane zahtjeve, a `PublicApi` za klijentske/javne zahtjeve.
3. **Null API ključ**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što će dovesti do 401 pogrešaka.