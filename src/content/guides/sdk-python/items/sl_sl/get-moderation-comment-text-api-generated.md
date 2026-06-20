## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_text_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_moderation_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_text_response import GetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je izbirno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (izbirno)

    try:
        api_response = api_instance.get_moderation_comment_text(comment_id, sso=sso)
        print("The response of ModerationApi->get_moderation_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment_text: %s\n" % e)
[inline-code-end]