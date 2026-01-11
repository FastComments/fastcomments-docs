## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| limit | number | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badge_progress_list200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_user_badge_progress_list'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badge_progress_list200_response import GetUserBadgeProgressList200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και εξ ορισμού είναι https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρακάτω δίνονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης, χρησιμοποιήστε αυτό που
# καλύπτει την περίπτωση χρήσης σας.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Μπείτε σε context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα παράδειγμα της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (προαιρετικό)
    limit = 3.4 # float |  (προαιρετικό)
    skip = 3.4 # float |  (προαιρετικό)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]