## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| includeEmail | boolean | query | Ne |  |
| includeIP | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_comment_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_moderation_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_comment_response import ModerationAPICommentResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_email = True # bool |  (optional)
    include_ip = True # bool |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_moderation_comment(comment_id, include_email=include_email, include_ip=include_ip, sso=sso)
        print("The response of ModerationApi->get_moderation_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment: %s\n" % e)
[inline-code-end]