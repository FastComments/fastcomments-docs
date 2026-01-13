## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| email | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_user_by_email_api_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_sso_user_by_email'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_user_by_email_api_response import GetSSOUserByEmailAPIResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και από προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να ρυθμίσει τις παραμέτρους πιστοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφάλειας του εξυπηρετητή API.
# Παρακάτω δίνονται παραδείγματα για κάθε μέθοδο πιστοποίησης, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί το σενάριο χρήσης πιστοποίησής σας.

# Διαμορφώστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    email = 'email_example' # str | 

    try:
        api_response = api_instance.get_sso_user_by_email(tenant_id, email)
        print("The response of DefaultApi->get_sso_user_by_email:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_user_by_email: %s\n" % e)
[inline-code-end]

---