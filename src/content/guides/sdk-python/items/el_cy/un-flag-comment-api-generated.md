## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Απόκριση

Επιστρέφει: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'un_flag_comment Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import UnFlagCommentOptions
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και έχει προεπιλογή https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους ελέγχου ταυτότητας και εξουσιοδότησης σύμφωνα με την πολιτική ασφαλείας του API server.
# Παραδείγματα για κάθε μέθοδο πιστοποίησης παρέχονται παρακάτω, χρησιμοποιήστε το παράδειγμα που καλύπτει την περίπτωσή σας.

# Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Απενεργοποιήστε το σχόλιο παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισαγωγή context με μια παρουσία του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία μιας παρουσίασης της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (προαιρετικό)
    anon_user_id = 'anon_user_id_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.un_flag_comment(tenant_id, id, UnFlagCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->un_flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_flag_comment: %s\n" % e)
[inline-code-end]