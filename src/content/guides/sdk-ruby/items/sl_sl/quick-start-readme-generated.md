### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Pred izvajanjem avtenticiranih zahtev morate nastaviti svoj API ključ v ApiClient. Če tega ne storite, bodo zahteve neuspešne z napako 401.

```ruby
require 'fastcomments-client'

# Ustvarite in konfigurirajte API odjemalca
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBVEZNO: Nastavite svoj API ključ (pridobite ga iz nadzorne plošče FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Ustvarite instanco API z konfiguriranim odjemalcem
api = FastCommentsClient::DefaultApi.new(api_client)

# Zdaj lahko izvajate avtenticirane klice API-ja
begin
  # Primer: Dodaj SSO uporabnika
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Pogoste napake:
  # - 401: API ključ manjka ali je neveljaven
  # - 400: Validacija zahteve ni uspela
end
```

### Uporaba javnih API-jev (PublicApi)

Javni končni točki ne zahtevajo avtentikacije:

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

### Pogoste težave

1. **401 "missing-api-key" napaka**: Prepričajte se, da nastavite `config.api_key['x-api-key'] = 'YOUR_KEY'` pred ustvarjanjem instance DefaultApi.
2. **Napačen razred API**: Uporabite `DefaultApi` za avtenticirane zahteve na strani strežnika, `PublicApi` za zahteve na strani odjemalca/javne zahteve.
3. **Null API ključ**: SDK bo tiho preskočil avtentikacijo, če je API ključ null, kar bo povzročilo napake 401.