### Korzystanie z uwierzytelnionych API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w ApiClient zanim wykonasz uwierzytelnione żądania. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```ruby
require 'fastcomments'

# Utwórz i skonfiguruj klienta API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# WYMAGANE: Ustaw swój klucz API (pobierz go z pulpitu FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Utwórz instancję API z skonfigurowanym klientem
api = FastCommentsClient::DefaultApi.new(api_client)

# Teraz możesz wykonywać uwierzytelnione wywołania API
begin
  # Przykład: Dodaj użytkownika SSO
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Najczęstsze błędy:
  # - 401: Brakuje klucza API lub jest nieprawidłowy
  # - 400: Walidacja żądania nie powiodła się
end
```

### Korzystanie z publicznych API (PublicApi)

Publiczne endpointy nie wymagają uwierzytelnienia:

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

### Korzystanie z API moderacji (ModerationApi)

Metody moderacji zasilają panel moderatora. Przekaż token `sso`, aby żądanie zostało wykonane w imieniu moderatora uwierzytelnionego przez SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Przykład: Pobierz listę komentarzy w kolejce moderacji
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Częste problemy

1. **Błąd 401 "missing-api-key"**: Upewnij się, że ustawisz `config.api_key['x-api-key'] = 'YOUR_KEY'` przed utworzeniem instancji DefaultApi.
2. **Nieprawidłowa klasa API**: Użyj `DefaultApi` dla uwierzytelnionych żądań po stronie serwera, `PublicApi` dla żądań po stronie klienta/publicznych oraz `ModerationApi` dla żądań panelu moderatora.
3. **Pusty klucz API (null)**: SDK cicho pominie uwierzytelnianie jeśli klucz API ma wartość null, co prowadzi do błędów 401.