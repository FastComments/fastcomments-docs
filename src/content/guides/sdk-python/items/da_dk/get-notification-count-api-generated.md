## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## Svar

Returnerer: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_notification_count Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationCountOptions
from client.models.get_notification_count_response import GetNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Definerer værten er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
# Klienten skal konfigurere godkendelses- og autorisationsparametre i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode er angivet nedenfor, brug det eksempel der opfylder dit godkendelsesbehov.
# Konfigurer API-nøgle autorisation: api_key
# Fjern kommentarer nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (valgfri)
    url_id = 'url_id_example' # str |  (valgfri)
    from_comment_id = 'from_comment_id_example' # str |  (valgfri)
    viewed = True # bool |  (valgfri)
    type = 'type_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_notification_count(tenant_id, GetNotificationCountOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type))
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]