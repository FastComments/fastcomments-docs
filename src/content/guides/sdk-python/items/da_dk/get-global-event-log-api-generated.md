req
tenantId
urlId
userIdWS

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| userIdWS | string | query | Yes |  |
| startTime | integer | query | Yes |  |
| endTime | integer | query | No |  |

## Svar

Returnerer: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_event_log_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_global_event_log'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_event_log_response import GetEventLogResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Opret en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id_ws = 'user_id_ws_example' # str | 
    start_time = 56 # int | 
    end_time = 56 # int |  (valgfri)

    try:
        api_response = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time=end_time)
        print("The response of PublicApi->get_global_event_log:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_global_event_log: %s\n" % e)
[inline-code-end]