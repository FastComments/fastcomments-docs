## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| deleteComments | boolean | query | Όχι |  |
| commentDeleteMode | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους πιστοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παραδείγματα για κάθε μέθοδο πιστοποίησης δίνονται παρακάτω, χρησιμοποιήστε το παράδειγμα που
# καλύπτει την περίπτωση χρήσης σας.

# Ρυθμίστε την εξουσιοδότηση με κλειδί API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε περιβάλλον (context) με ένα στιγμιότυπο του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (προαιρετικό)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]