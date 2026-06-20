### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Pred izvajanjem avtenticiranih zahtev morate nastaviti svoj API ključ na ApiClientu. Če tega ne storite, bodo zahteve neuspešne s 401 napako.

```ruby
require 'fastcomments'

# Ustvari in konfiguriraj API odjemalca
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBVEZNO: Nastavite svoj API ključ (dobite ga na nadzorni plošči FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Ustvari instanco API-ja z konfiguriranim odjemalcem
api = FastCommentsClient::DefaultApi.new(api_client)

# Zdaj lahko izvajate overjene klice API-ja
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
  # - 401: API ključ manjka ali ni veljaven
  # - 400: Preverjanje zahtevka ni uspelo
end
```

### Uporaba javnih API-jev (PublicApi)

Javni končni točki ne zahtevata avtentikacije:

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

### Uporaba moderacijskih API-jev (ModerationApi)

Metode moderiranja poganjajo nadzorno ploščo moderatorja. Posredujte žeton `sso`, da bo zahteva izvedena v imenu moderatorja, avtenticiranega prek SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Primer: Naštej komentarje v vrsti za moderiranje
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Pogoste težave

1. **401 "missing-api-key" error**: Prepričajte se, da nastavite `config.api_key['x-api-key'] = 'YOUR_KEY'` pred ustvarjanjem instance DefaultApi.
2. **Wrong API class**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorja.
3. **Null API key**: SDK bo tiho preskočil avtentikacijo, če je API ključ null, kar bo povzročilo 401 napake.