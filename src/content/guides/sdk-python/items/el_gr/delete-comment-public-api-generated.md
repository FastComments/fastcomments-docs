## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| broadcastId | string | query | Ναι |  |
| editKey | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_delete_comment_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_api_delete_comment_response import PublicAPIDeleteCommentResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και η προεπιλεγμένη τιμή είναι https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα αντικείμενο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα αντικείμενο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.delete_comment_public(tenant_id, comment_id, broadcast_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->delete_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_public: %s\n" % e)
[inline-code-end]