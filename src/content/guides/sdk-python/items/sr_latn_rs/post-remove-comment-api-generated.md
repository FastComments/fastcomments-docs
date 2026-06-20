---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Response

Vraća: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/post_remove_comment_response.py)

## Primer

[inline-code-attrs-start title = 'post_remove_comment Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.post_remove_comment_response import PostRemoveCommentResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.post_remove_comment(comment_id, sso=sso)
        print("The response of ModerationApi->post_remove_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_remove_comment: %s\n" % e)
[inline-code-end]

---