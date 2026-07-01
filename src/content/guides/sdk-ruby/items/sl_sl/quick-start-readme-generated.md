### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Morate nastaviti svoj API ključ v ApiClient, preden izvedete avtenticirane zahteve. Če tega ne storite, bodo zahteve propadle z napako 401.

```ruby
require 'fastcomments'

# Ustvari in konfiguriraj API odjemalca
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ZAHTEVANO: Nastavite svoj API ključ (pridobite ga v nadzorni plošči FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Ustvari instanco API-ja s konfiguriranim odjemalcem
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
  # - 400: Validacija zahteve je spodletela
end
```

### Uporaba javnih API-jev (PublicApi)

Javni končni naslovi ne zahtevajo avtentikacije:

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

### Uporaba moderacijskih API-jev (ModerationApi)

Metode moderacije poganjajo nadzorno ploščo moderatorjev. Posredujte `sso` žeton, da je zahteva izvedena v imenu SSO‑avtenticiranega moderatorja:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Primer: Naštej komentarje v moderacijski čakalni vrsti
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Pogoste težave

1. **401 napaka “missing-api-key”**: Prepričajte se, da ste pred ustvarjanjem instance DefaultApi nastavili `config.api_key['x-api-key'] = 'YOUR_KEY'`.
2. **Napačen API razred**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorja.
3. **Null API ključ**: SDK bo tiho preskočil avtentikacijo, če je API ključ `null`, kar povzroči napake 401.