### Using Authenticated APIs (DefaultApi)

**Important:** You must set your API key on the ApiClient before making authenticated requests. If you don't, requests will fail with a 401 error.

```ruby
require 'fastcomments'

# Δημιουργία και ρύθμιση του πελάτη API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ΑΠΑΙΤΗΤΟ: Ορίστε το κλειδί API σας (πάρτε το από τον πίνακα FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Δημιουργία του αντικειμένου API με τον διαμορφωμένο πελάτη
api = FastCommentsClient::DefaultApi.new(api_client)

# Τώρα μπορείτε να κάνετε αυθεντικοποιημένες κλήσεις API
begin
  # Παράδειγμα: Προσθήκη χρήστη SSO
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Συνηθισμένα σφάλματα:
  # - 401: Το κλειδί API λείπει ή είναι μη έγκυρο
  # - 400: Αποτυχία επικύρωσης του αιτήματος
end
```

### Using Public APIs (PublicApi)

Public endpoints don't require authentication:

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

### Using Moderation APIs (ModerationApi)

The moderation methods power the moderator dashboard. Pass an `sso` token so the request is made on behalf of an SSO-authenticated moderator:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Παράδειγμα: Καταγραφή σχολίων στην ουρά συντονισμού
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Common Issues

1. **401 "missing-api-key" error**: Make sure you set `config.api_key['x-api-key'] = 'YOUR_KEY'` before creating the DefaultApi instance.
2. **Wrong API class**: Use `DefaultApi` for server-side authenticated requests, `PublicApi` for client-side/public requests, and `ModerationApi` for moderator dashboard requests.
3. **Null API key**: The SDK will silently skip authentication if the API key is null, leading to 401 errors.