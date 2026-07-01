### Korišćenje autentifikovanih API‑ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ na ApiClient prije nego što napravite autentifikovane zahteve. Ako to ne učinite, zahtevi će propasti s greškom 401.

```ruby
require 'fastcomments'

# Kreirajte i konfigurišite API klijent
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Postavite vaš API ključ (preuzmite ga sa FastComments kontrolne table)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreirajte API instancu sa konfigurisanim klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete praviti autentifikovane API pozive
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
  # - 401: API ključ nedostaje ili je neispravan
  # - 400: Validacija zahteva nije uspjela
end
```

### Korišćenje javnih API‑ja (PublicApi)

Javni endpointi ne zahtevaju autentifikaciju:

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

### Korišćenje moderacionih API‑ja (ModerationApi)

Metode za moderaciju omogućavaju rad moderatorske table. Proslijedite `sso` token kako bi se zahtev izvršio u ime SSO‑autentifikovanog moderatora:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Primer: Prikaz komentara u moderacionom redu
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Uobičajeni problemi

1. **401 "missing-api-key" error**: Uverite se da ste postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` prije kreiranja DefaultApi instance.
2. **Wrong API class**: Koristite `DefaultApi` za server‑side autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve i `ModerationApi` za zahteve moderatora na dashboardu.
3. **Null API key**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što dovodi do 401 grešaka.