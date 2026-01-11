req
tenantId
urlId
userIdWS

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| userIdWS | string | query | Sí |  |
| startTime | integer | query | Sí |  |
| endTime | integer | query | Sí |  |

## Respuesta

Devuelve: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_event_log200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_event_log'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_event_log200_response import GetEventLog200Response
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id_ws = 'user_id_ws_example' # str | 
    start_time = 56 # int | 
    end_time = 56 # int | 

    try:
        api_response = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)
        print("The response of PublicApi->get_event_log:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_event_log: %s\n" % e)
[inline-code-end]