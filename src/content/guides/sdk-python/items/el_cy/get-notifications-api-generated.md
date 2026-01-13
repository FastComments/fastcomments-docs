## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| urlId | string | query | Όχι |  |
| fromCommentId | string | query | Όχι |  |
| viewed | boolean | query | Όχι |  |
| type | string | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notifications200_response import GetNotifications200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και από προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο πελάτης πρέπει να διαμορφώσει τις παραμέτρους πιστοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο πιστοποίησης, χρησιμοποιήστε
# το παράδειγμα που καλύπτει την περίπτωσή σας.

# Διαμόρφωση εξουσιοδότησης με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν απαιτείται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    from_comment_id = 'from_comment_id_example' # str |  (optional)
    viewed = True # bool |  (optional)
    type = 'type_example' # str |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_notifications(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip)
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]