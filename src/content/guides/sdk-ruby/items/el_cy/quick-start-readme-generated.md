### Χρήση Αυθεντικοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στο ApiClient πριν κάνετε αυθεντικοποιημένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```ruby
require 'fastcomments'

# Δημιουργία και ρύθμιση του πελάτη API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το κλειδί API σας (λάβετε το από τον πίνακα ελέγχου του FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Δημιουργήστε το instance του API με τον ρυθμισμένο client
api = FastCommentsClient::DefaultApi.new(api_client)

# Τώρα μπορείτε να πραγματοποιήσετε αυθεντικοποιημένες κλήσεις API
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
  # Συνήθη σφάλματα:
  # - 401: Το κλειδί API λείπει ή είναι άκυρο
  # - 400: Αποτυχία επικύρωσης αιτήματος
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

### Χρήση API Εποπτείας (ModerationApi)

Οι μέθοδοι εποπτείας τροφοδοτούν τον πίνακα ελέγχου του συντονιστή. Δώστε ένα `sso` token ώστε το αίτημα να γίνει εξ ονόματος ενός συντονιστή που έχει αυθεντικοποιηθεί μέσω SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Παράδειγμα: Λίστα σχολίων στην ουρά εποπτείας
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Συνηθισμένα Προβλήματα

1. **401 "missing-api-key" σφάλμα**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key['x-api-key'] = 'YOUR_KEY'` πριν δημιουργήσετε το instance DefaultApi.
2. **Λάθος κλάση API**: Χρησιμοποιήστε `DefaultApi` για αιτήματα στο server που απαιτούν αυθεντικοποίηση, `PublicApi` για client-side/δημόσια αιτήματα, και `ModerationApi` για αιτήματα του πίνακα ελέγχου συντονιστή.
3. **Κενό κλειδί API**: Το SDK θα παραλείψει σιωπηλά την αυθεντικοποίηση αν το κλειδί API είναι null, οδηγώντας σε σφάλματα 401.