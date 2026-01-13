## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'replace_tenant_package Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.replace_tenant_package_body import ReplaceTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένο είναι το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παραδείγματα για κάθε μέθοδο αυθεντικοποίησης παρέχονται παρακάτω, χρησιμοποιήστε το
# παράδειγμα που ικανοποιεί την περίπτωσή σας.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αφαιρέστε το σχόλιο παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε πλαίσιο με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    replace_tenant_package_body = client.ReplaceTenantPackageBody() # ReplaceTenantPackageBody | 

    try:
        api_response = api_instance.replace_tenant_package(tenant_id, id, replace_tenant_package_body)
        print("The response of DefaultApi->replace_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->replace_tenant_package: %s\n" % e)
[inline-code-end]