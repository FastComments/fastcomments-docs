### Χρήση Πιστοποιημένων APIs (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το API key σας στο ApiClient πριν κάνετε πιστοποιημένες κλήσεις. Εάν δεν το κάνετε, οι αιτήσεις θα αποτύχουν με σφάλμα 401.

```ruby
require 'fastcomments-client'

# Δημιουργία και ρύθμιση του πελάτη API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το API key σας (λάβετε το από τον πίνακα ελέγχου FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Δημιουργία του instance του API με τον διαμορφωμένο client
api = FastCommentsClient::DefaultApi.new(api_client)

# Τώρα μπορείτε να κάνετε πιστοποιημένες κλήσεις API
begin
  # Παράδειγμα: Προσθήκη SSO χρήστη
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Συνήθη σφάλματα:
  # - 401: Το API key λείπει ή είναι άκυρο
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

### Συνήθη προβλήματα

1. **401 "missing-api-key" error**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key['x-api-key'] = 'YOUR_KEY'` πριν δημιουργήσετε το instance `DefaultApi`.
2. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για server-side πιστοποιημένα αιτήματα, `PublicApi` για client-side/δημόσια αιτήματα.
3. **Null API key**: Το SDK θα παραλείψει αθόρυβα την αυθεντικοποίηση εάν το API key είναι null, οδηγώντας σε σφάλματα 401.