### Korzystanie z uwierzytelnionych API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w ApiClient przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```ruby
require 'fastcomments-client'

# Utwórz i skonfiguruj klienta API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# WYMAGANE: Ustaw swój klucz API (pobierz go z panelu FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Utwórz instancję API z użyciem skonfigurowanego klienta
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
  # Typowe błędy:
  # - 401: Klucz API jest brakujący lub nieprawidłowy
  # - 400: Walidacja żądania nie powiodła się
end
```

### Korzystanie z publicznych API (PublicApi)

Publiczne endpointy nie wymagają uwierzytelnienia:

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

### Częste problemy

1. **Błąd 401 "missing-api-key"**: Upewnij się, że ustawisz `config.api_key['x-api-key'] = 'YOUR_KEY'` przed utworzeniem instancji DefaultApi.
2. **Nieprawidłowa klasa API**: Użyj `DefaultApi` dla uwierzytelnionych żądań po stronie serwera, `PublicApi` dla żądań po stronie klienta/publicznych.
3. **Pusty klucz API**: SDK pominie uwierzytelnianie bez komunikatu, jeśli klucz API jest null, co prowadzi do błędów 401.