## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentIds | string | query | Ja | En kommasepareret liste med kommentar-id'er. |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/checked_comments_for_blocked200_response.py)

## Eksempel

[inline-code-attrs-start title = 'checked_comments_for_blocked Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.checked_comments_for_blocked200_response import CheckedCommentsForBlocked200Response
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | En kommasepareret liste med kommentar-id'er.
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]