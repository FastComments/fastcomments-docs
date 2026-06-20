## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'un_flag_comment Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και η προεπιλεγμένη τιμή είναι https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους authentication και authorization
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο auth — χρησιμοποιήστε αυτό
# που καλύπτει την περίπτωση χρήσης σας.

# Διαμόρφωση εξουσιοδότησης με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)

    try:
        api_response = api_instance.un_flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->un_flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_flag_comment: %s\n" % e)
[inline-code-end]