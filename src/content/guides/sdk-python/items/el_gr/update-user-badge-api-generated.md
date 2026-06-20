## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Απόκριση

Επιστρέφει: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_success_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_user_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_success_response import APIEmptySuccessResponse
from client.models.update_user_badge_params import UpdateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και έχει προεπιλεγμένη τιμή https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης — χρησιμοποιήστε
# το παράδειγμα που ταιριάζει στην περίπτωσή σας.
# Διαμορφώστε την εξουσιοδότηση με API key: api_key
# Αποσχολιάστε την παρακάτω γραμμή για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_user_badge_params = client.UpdateUserBadgeParams() # UpdateUserBadgeParams | 

    try:
        api_response = api_instance.update_user_badge(tenant_id, id, update_user_badge_params)
        print("The response of DefaultApi->update_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_user_badge: %s\n" % e)
[inline-code-end]