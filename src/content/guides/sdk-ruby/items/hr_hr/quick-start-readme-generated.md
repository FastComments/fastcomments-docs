### Korištenje autentificiranih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ na ApiClient prije izvođenja autentificiranih zahtjeva. Ako to ne učinite, zahtjevi će vratiti grešku 401.

```ruby
require 'fastcomments'

# Kreiraj i konfiguriraj API klijent
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Postavite vaš API ključ (nabavite ga sa FastComments nadzorne ploče)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreiraj API instancu s konfiguriranim klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete izvršavati autentificirane API pozive
begin
  # Primjer: Dodaj SSO korisnika
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
  # - 401: API ključ nedostaje ili nije valjan
  # - 400: Neuspjela validacija zahtjeva
end
```

### Korištenje javnih API-ja (PublicApi)

Javni endpointi ne zahtijevaju autentikaciju:

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

### Korištenje moderacijskih API-ja (ModerationApi)

Metode za moderaciju pokreću nadzornu ploču moderatora. Proslijedite `sso` token kako bi se zahtjev napravio u ime moderatora autentificiranog putem SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Primjer: Popis komentara u redu za moderaciju
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Česti problemi

1. **401 "missing-api-key" greška**: Provjerite jeste li postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` prije kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server-side autentificirane zahtjeve, `PublicApi` za klijentske/javne zahtjeve, i `ModerationApi` za zahtjeve nadzorne ploče moderatora.
3. **Null API ključ**: SDK će tiho preskočiti autentikaciju ako je API ključ null, što će dovesti do 401 grešaka.