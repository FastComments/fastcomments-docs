### Χρήση Πιστοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στη Διαμόρφωση (Configuration) πριν κάνετε πιστοποιημένα αιτήματα. Εάν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### Χρήση Δημόσιων API (PublicApi)

Τα δημόσια σημεία άκρου δεν απαιτούν πιστοποίηση:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση Πίνακα Ελέγχου Συντονισμού (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου του συντονιστή. Οι μέθοδοι καλούνται εκ μέρους ενός συντονιστή με τη μεταβίβαση ενός διακριτικού `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση SSO (Single Sign-On)

Το SDK περιλαμβάνει βοηθητικά εργαλεία για τη δημιουργία ασφαλών διακριτικών SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data (id, email, and username are required)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Sign it with your API secret (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generate the SSO token to pass to the widget or an API call
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

Για απλό SSO (λιγότερο ασφαλές, για δοκιμές):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Ζωντανές Συνδρομές (PubSub)

Το module `pubsub` σας επιτρέπει να εγγραφείτε σε γεγονότα σχολίων σε πραγματικό χρόνο (νέα σχόλια, ψήφοι, επεξεργασίες, ειδοποιήσεις κ.λπ.) μέσω WebSocket, αντικατοπτρίζοντας το `LiveEventSubscriber` του FastComments Java SDK. Απαιτεί το πρόσθετο `pubsub`, το οποίο προσθέτει έναν πελάτη WebSocket πάνω από τον παραγόμενο πελάτη API:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # e.g. a UUID for this session
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # set to "eu" for the EU region
)

# ...later, when you no longer want updates:
result.close()
```

Ο συνδρομητής εκτελεί τη σύνδεση σε ένα νήμα daemon στο παρασκήνιο, επανασυνδέεται διαφανώς με jitter, και ανακτά τυχόν γεγονότα που χάθηκαν ενώ ήταν αποσυνδεδεμένος από το endpoint event‑log κατά την επανασύνδεση. Περνάτε ένα προαιρετικό callback `can_see_comments` (`List[str] -> Dict[str, str]`, που επιστρέφει τα IDs που ο χρήστης ΔΕΝ μπορεί να δει) για να φιλτράρετε τα γεγονότα για σχόλια που ο χρήστης δεν επιτρέπεται να δει. Ορίστε `disable_live_commenting=True` για να κάνετε το `subscribe_to_changes` μια λειτουργία no‑op που επιστρέφει `None`.

### Κοινά Προβλήματα

1. **401 "missing-api-key" error** Make sure you set `config.api_key = {"api_key": "YOUR_KEY"}` before creating the DefaultApi instance.
2. **Wrong API class** Use `DefaultApi` for server-side authenticated requests, `PublicApi` for client-side/public requests, and `ModerationApi` for moderator dashboard requests.
3. **Import errors** Make sure you're importing from the correct module:
   - API client: `from client import ...`
   - SSO utilities: `from sso import ...`
   - Live subscriptions: `from pubsub import ...` (needs the `pubsub` extra)