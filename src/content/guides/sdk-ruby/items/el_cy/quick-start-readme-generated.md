### Χρήση Αυθεντικοποιημένων APIs (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στο ApiClient πριν κάνετε αυθεντικοποιημένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```ruby
require 'fastcomments-client'

# Δημιουργία και ρύθμιση του ApiClient
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το κλειδί API σας (λάβετε το από τον πίνακα ελέγχου FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Δημιουργία του αντικειμένου API με τον διαμορφωμένο πελάτη
api = FastCommentsClient::DefaultApi.new(api_client)

# Τώρα μπορείτε να κάνετε αυθεντικοποιημένες κλήσεις στο API
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
  # - 401: Το κλειδί API λείπει ή είναι άκυρο
  # - 400: Η επικύρωση του αιτήματος απέτυχε
end
```

### Χρήση Δημόσιων APIs (PublicApi)

Τα δημόσια endpoints δεν απαιτούν αυθεντικοποίηση:

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

### Συνηθισμένα Προβλήματα

1. **401 "missing-api-key" error**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key['x-api-key'] = 'YOUR_KEY'` πριν δημιουργήσετε το αντικείμενο DefaultApi.
2. **Λανθασμένη κλάση API**: Χρησιμοποιήστε `DefaultApi` για αιτήματα που γίνονται από τον διακομιστή με αυθεντικοποίηση, και `PublicApi` για αιτήματα από την πλευρά του πελάτη/δημόσια.
3. **Κενό API key**: Το SDK θα παραλείψει σιωπηλά την αυθεντικοποίηση αν το κλειδί API είναι null, οδηγώντας σε σφάλματα 401.