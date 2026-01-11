## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| editKey | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_text200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_text200_response import GetCommentText200Response
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je izbirno in privzeto https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (izbirno)
    sso = 'sso_example' # str |  (izbirno)

    try:
        api_response = api_instance.get_comment_text(tenant_id, comment_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->get_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]

---