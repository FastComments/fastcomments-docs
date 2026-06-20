## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Odgovor

Vrne: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/post_remove_comment_response.py)

## Primer

[inline-code-attrs-start title = 'Primer post_remove_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.post_remove_comment_response import PostRemoveCommentResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco odjemalca API
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.post_remove_comment(comment_id, sso=sso)
        print("The response of ModerationApi->post_remove_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_remove_comment: %s\n" % e)
[inline-code-end]

---