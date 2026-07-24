### Χρήση Πιστοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στη Διαμόρφωση (Configuration) πριν κάνετε πιστοποιημένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Δημιουργία και διαμόρφωση του πελάτη API
config = Configuration()
config.host = "https://fastcomments.com"

# ΑΠΑΙΤΟΥΜΕΝΟ: Ορίστε το κλειδί API σας (πάρτε το από τον πίνακα ελέγχου FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Παράδειγμα: Προσθήκη χρήστη SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Συνηθισμένα σφάλματα:
    # - 401: Το κλειδί API λείπει ή είναι μη έγκυρο
    # - 400: Η επικύρωση του αιτήματος απέτυχε
```

### Χρήση Δημόσιων API (PublicApi)

Τα δημόσια σημεία άκρης δεν απαιτούν πιστοποίηση:

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

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου του συντονιστή. Οι μέθοδοι καλούνται εκ μέρους ενός συντονιστή περνώντας ένα διακριτικό `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Καταμέτρηση των σχολίων που αναμένουν συντονισμό
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση SSO (Μοναδική Σύνδεση)

Το SDK περιλαμβάνει βοηθητικά εργαλεία για τη δημιουργία ασφαλών διακριτικών SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Δημιουργία δεδομένων χρήστη (απαιτούνται id, email και όνομα χρήστη)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Υπογράψτε το με το μυστικό API σας (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Δημιουργία του διακριτικού SSO για να το περάσετε στο widget ή σε κλήση API
sso_token = sso.create_token()

# Χρησιμοποιήστε αυτό το διακριτικό στο frontend σας ή περάστε το σε κλήσεις API
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

Το module `pubsub` σας επιτρέπει να εγγραφείτε σε πραγματικού χρόνου γεγονότα σχολίων (νέα σχόλια, ψήφοι, επεξεργασίες, ειδοποιήσεις κ.λπ.) μέσω WebSocket, αντικατοπτρίζοντας το `LiveEventSubscriber` του FastComments Java SDK. Απαιτεί το πρόσθετο `pubsub`, το οποίο προσθέτει έναν πελάτη WebSocket πάνω από τον παραγόμενο πελάτη API:

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
    user_id_ws="a-unique-presence-id",  # π.χ. ένα UUID για αυτή τη συνεδρία
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # ορίστε σε "eu" για την περιοχή EU
)

# ...αργότερα, όταν δεν θέλετε πλέον ενημερώσεις:
result.close()
```

Ο συνδρομητής εκτελεί τη σύνδεση σε ένα νήμα daemon στο παρασκήνιο, επανασυνδέεται διαφανώς με jitter, και ανακτά τυχόν γεγονότα που χάθηκαν ενώ ήταν αποσυνδεδεμένος από το endpoint του event‑log κατά την επανασύνδεση. Περνάτε ένα προαιρετικό callback `can_see_comments` (`List[str] -> Dict[str, str]`, επιστρέφει τα ids που ο χρήστης ΔΕΝ μπορεί να δει) για να φιλτράρετε τα γεγονότα για σχόλια που ο χρήστης δεν επιτρέπεται να δει. Ορίστε `disable_live_commenting=True` για να κάνετε το `subscribe_to_changes` ένα no‑op που επιστρέφει `None`.

### Συνηθισμένα Προβλήματα

1. **Σφάλμα 401 "missing-api-key"**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key = {"api_key": "YOUR_KEY"}` πριν δημιουργήσετε το αντικείμενο DefaultApi.
2. **Λάθος κλάση API**: Χρησιμοποιήστε `DefaultApi` για αιτήματα πιστοποιημένα από τον διακομιστή, `PublicApi` για αιτήματα από τον πελάτη/δημόσια, και `ModerationApi` για αιτήματα πίνακα ελέγχου συντονιστή.
3. **Σφάλματα εισαγωγής**: Βεβαιωθείτε ότι κάνετε εισαγωγή από το σωστό module:
   - Πελάτης API: `from client import ...`
   - Βοηθητικά SSO: `from sso import ...`
   - Ζωντανές συνδρομές: `from pubsub import ...` (απαιτεί το πρόσθετο `pubsub`)