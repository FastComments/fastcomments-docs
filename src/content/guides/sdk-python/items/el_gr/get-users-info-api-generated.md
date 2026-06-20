Συγκεντρωμένες πληροφορίες χρηστών για έναν tenant. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από User / SSOUser.
Χρησιμοποιείται από το comment widget για να εμπλουτίσει χρήστες που μόλις εμφανίστηκαν μέσω ενός presence event.
Χωρίς context σελίδας: η ιδιωτικότητα εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ αποκρύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds διαχωρισμένα με κόμμα. |

## Απόκριση

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_users_info'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | userIds διαχωρισμένα με κόμμα.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]