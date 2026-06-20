## Παράμετροι

| Όνομα | Τύπος | Θέση | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| urlId | string | query | Όχι |  |
| fromCommentId | string | query | Όχι |  |
| viewed | boolean | query | Όχι |  |
| type | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'get_notification_count Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notification_count_response import GetNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Ο client πρέπει να ρυθμίσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.
# Παρακάτω δίνονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης, χρησιμοποιήστε το
# παράδειγμα που ικανοποιεί την περίπτωσή χρήσης σας.

# Configure API key authorization: api_key
# Διαμορφώστε την εξουσιοδότηση με API κλειδί: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (προαιρετικό)
    url_id = 'url_id_example' # str |  (προαιρετικό)
    from_comment_id = 'from_comment_id_example' # str |  (προαιρετικό)
    viewed = True # bool |  (προαιρετικό)
    type = 'type_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.get_notification_count(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type)
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]