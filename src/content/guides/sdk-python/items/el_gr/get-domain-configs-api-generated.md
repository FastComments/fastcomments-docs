## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_configs_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_domain_configs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_configs_response import GetDomainConfigsResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να ρυθμίσει τις παραμέτρους authentication και authorization
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παραδείγματα για κάθε μέθοδο auth δίνονται παρακάτω, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωση χρήσης auth σας.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Καταργήστε το σχόλιο στην παρακάτω γραμμή για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_domain_configs(tenant_id)
        print("The response of DefaultApi->get_domain_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_configs: %s\n" % e)
[inline-code-end]