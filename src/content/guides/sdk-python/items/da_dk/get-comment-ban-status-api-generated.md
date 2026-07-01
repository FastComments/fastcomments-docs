## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Response

Returnerer: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_ban_status_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_comment_ban_status Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_ban_status_response import GetCommentBanStatusResponse
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Indtast en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_comment_ban_status(tenant_id, comment_id, sso=sso)
        print("The response of ModerationApi->get_comment_ban_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_comment_ban_status: %s\n" % e)
[inline-code-end]