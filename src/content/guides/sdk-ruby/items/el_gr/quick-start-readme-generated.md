---
### Χρήση Επαληθευμένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API στο ApiClient πριν κάνετε επαληθευμένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```ruby
require 'fastcomments'

# Δημιουργήστε και ρυθμίστε τον API client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το κλειδί API σας (το λάβετε από τον πίνακα ελέγχου FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Δημιουργήστε το instance του API με τον ρυθμισμένο client
api = FastCommentsClient::DefaultApi.new(api_client)

# Τώρα μπορείτε να κάνετε επαληθευμένες κλήσεις API
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
  # - 400: Η επικύρωση αιτήματος απέτυχε
end
```

### Χρήση Δημόσιων API (PublicApi)

Τα δημόσια endpoints δεν απαιτούν αυθεντικοποίηση:

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

### Χρήση API Ελέγχου (ModerationApi)

Οι μέθοδοι moderation τροφοδοτούν τον πίνακα ελέγχου των συντονιστών. Δώστε ένα `sso` token ώστε το αίτημα να γίνει εκ μέρους ενός συντονιστή που έχει πιστοποιηθεί μέσω SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Παράδειγμα: Λίστα σχολίων στην ουρά επιτήρησης
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Συνηθισμένα Προβλήματα

1. **401 "missing-api-key" error**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key['x-api-key'] = 'YOUR_KEY'` πριν δημιουργήσετε το instance DefaultApi.
2. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για server-side επαληθευμένα αιτήματα, `PublicApi` για client-side/δημόσια αιτήματα, και `ModerationApi` για αιτήματα του πίνακα ελέγχου συντονιστή.
3. **Null API key**: Το SDK θα παραλείψει σιωπηρά την αυθεντικοποίηση εάν το κλειδί API είναι null, οδηγώντας σε σφάλματα 401.
---