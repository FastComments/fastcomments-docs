## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentIds | string | query | Da | Seznam id-jev komentarjev, ločen z vejicami. |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/checked_comments_for_blocked200_response.py)

## Primer

[inline-code-attrs-start title = 'checked_comments_for_blocked Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.checked_comments_for_blocked200_response import CheckedCommentsForBlocked200Response
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | Seznam id-jev komentarjev, ločen z vejicami.
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]