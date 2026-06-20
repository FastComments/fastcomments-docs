## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| broadcastId | string | query | Ναι |  |
| editKey | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_set_comment_text_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comment_text_update_request import CommentTextUpdateRequest
from client.models.public_api_set_comment_text_response import PublicAPISetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και έχει προεπιλεγμένη τιμή https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του ApiClient
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_text_update_request = client.CommentTextUpdateRequest() # CommentTextUpdateRequest | 
    edit_key = 'edit_key_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->set_comment_text: %s\n" % e)
[inline-code-end]