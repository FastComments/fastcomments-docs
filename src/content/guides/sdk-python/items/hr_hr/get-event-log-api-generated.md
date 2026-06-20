req
tenantId
urlId
userIdWS

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| urlId | string | upit | Da |  |
| userIdWS | string | upit | Da |  |
| startTime | integer | upit | Da |  |
| endTime | integer | upit | Ne |  |

## Odgovor

Vraća: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_event_log_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_event_log'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_event_log_response import GetEventLogResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id_ws = 'user_id_ws_example' # str | 
    start_time = 56 # int | 
    end_time = 56 # int |  (neobavezno)

    try:
        api_response = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time=end_time)
        print("The response of PublicApi->get_event_log:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_event_log: %s\n" % e)
[inline-code-end]