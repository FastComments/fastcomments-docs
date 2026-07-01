### Χρήση Πιστοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στο ApiClient πριν κάνετε αιτήματα με πιστοποίηση. Εάν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```ruby
require 'fastcomments'

# Create and configure the API client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Create the API instance with the configured client
api = FastCommentsClient::DefaultApi.new(api_client)

# Now you can make authenticated API calls
begin
  # Example: Add an SSO user
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Common errors:
  # - 401: API key is missing or invalid
  # - 400: Request validation failed
end
```

### Χρήση Δημόσιων API (PublicApi)

Τα δημόσια σημεία άκρης δεν απαιτούν πιστοποίηση:

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

### Χρήση APIs Μετριασμού (ModerationApi)

Οι μέθοδοι μετριασμού τροφοδοτούν τον πίνακα ελέγχου του συντονιστή. Περνάτε ένα διακριτικό `sso` ώστε το αίτημα να γίνει εκ μέρους ενός συντονιστή πιστοποιημένου μέσω SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Example: List comments in the moderation queue
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Κοινά Προβλήματα

1. **401 "missing-api-key" error**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key['x-api-key'] = 'YOUR_KEY'` πριν δημιουργήσετε την παρουσία DefaultApi.
2. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για αιτήματα διακομιστή με πιστοποίηση, `PublicApi` για αιτήματα πελάτη/δημόσια, και `ModerationApi` για αιτήματα πίνακα ελέγχου συντονιστή.
3. **Null API key**: Το SDK θα παραλείψει σιωπηλά την πιστοποίηση αν το κλειδί API είναι κενό, οδηγώντας σε σφάλματα 401.