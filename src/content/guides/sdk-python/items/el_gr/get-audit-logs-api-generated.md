## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| limit | number | query | Όχι |  |
| skip | number | query | Όχι |  |
| order | string | query | Όχι |  |
| after | number | query | Όχι |  |
| before | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_audit_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs200_response import GetAuditLogs200Response
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει ως προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης. Χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί τη χρήση αυθεντικοποίησής σας.

# Ρύθμιση εξουσιοδότησης με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε πλαίσιο με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (προαιρετικό)
    skip = 3.4 # float |  (προαιρετικό)
    order = client.SORTDIR() # SORTDIR |  (προαιρετικό)
    after = 3.4 # float |  (προαιρετικό)
    before = 3.4 # float |  (προαιρετικό)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]