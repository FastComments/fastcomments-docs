### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ na ApiClient prije nego što napravite autentifikovane zahtjeve. Ako to ne uradite, zahtjevi će završiti sa greškom 401.

```ruby
require 'fastcomments'

# Kreirajte i konfigurišite API klijent
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBAVEZNO: Postavite vaš API ključ (preuzmite ga iz FastComments nadzorne ploče)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Kreirajte instancu API-ja sa konfigurisanom klijentom
api = FastCommentsClient::DefaultApi.new(api_client)

# Sada možete izvršavati autentifikovane pozive API-ja
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
  # Česte greške:
  # - 401: API ključ nedostaje ili nije validan
  # - 400: Validacija zahtjeva nije uspjela
end
```

### Korišćenje javnih API-ja (PublicApi)

Javni endpointi ne zahtevaju autentifikaciju:

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

### Korišćenje API-ja za moderaciju (ModerationApi)

Metode za moderaciju pokreću moderatorsku nadzornu ploču. Prosledite `sso` token kako bi zahtev bio izvršen u ime moderatora autentifikovanog putem SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Primer: Prikaz komentara u redu za moderaciju
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Uobičajeni problemi

1. **401 "missing-api-key" error**: Uverite se da ste postavili `config.api_key['x-api-key'] = 'YOUR_KEY'` pre kreiranja DefaultApi instance.
2. **Wrong API class**: Koristite `DefaultApi` za serverske autentifikovane zahtjeve, `PublicApi` za klijentske/javne zahtjeve, i `ModerationApi` za zahtjeve moderatorske nadzorne ploče.
3. **Null API key**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što će dovesti do 401 grešaka.