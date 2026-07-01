### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ na ApiClient prije nego što napravite autentifikovane zahtjeve. Ako to ne učinite, zahtjevi će propasti s greškom 401.

```ruby
require 'fastcomments'

# Kreirajte i konfigurišite API klijent
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Postavite vaš API ključ (uzmite ga iz vašeg FastComments kontrolnog panela)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreirajte API instancu sa konfiguriranim klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete napraviti autentifikovane API pozive
begin
  # Primer: Dodajte SSO korisnika
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
  # - 401: API ključ nedostaje ili je neispravan
  # - 400: Validacija zahtjeva nije uspjela
end
```

### Korišćenje javnih API-ja (PublicApi)

Javni endpointi ne zahtijevaju autentifikaciju:

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

### Korišćenje moderacijskih API-ja (ModerationApi)

Metode moderacije napajaju moderator panel. Proslijedite `sso` token tako da se zahtjev izvrši u ime moderatora autentifikovanog putem SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Primer: Izlistajte komentare u moderacijskom redu
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Uobičajeni problemi

1. **401 "missing-api-key" greška**: Provjerite da ste postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` prije kreiranja DefaultApi instance.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahtjeve, `PublicApi` za klijentske/javne zahtjeve i `ModerationApi` za zahtjeve moderator panela.
3. **Null API ključ**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što rezultira 401 greškama.