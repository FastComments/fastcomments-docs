## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| approved | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_approved_response.py)

## Primer

[inline-code-attrs-start title = 'Primer post_set_comment_approval_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_approved_response import SetCommentApprovedResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    approved = True # bool |  (neobvezno)
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.post_set_comment_approval_status(comment_id, approved=approved, sso=sso)
        print("The response of ModerationApi->post_set_comment_approval_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_approval_status: %s\n" % e)
[inline-code-end]