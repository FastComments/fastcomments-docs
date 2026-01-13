## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## Respuesta

Devuelve: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count200_response import GetPendingWebhookEventCount200Response
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para obtener una lista de todos los parámetros de configuración admitidos.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor de la API.
# Se proporcionan ejemplos para cada método de autenticación a continuación, use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configurar autorización por clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrar en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (opcional)
    external_id = 'external_id_example' # str |  (opcional)
    event_type = 'event_type_example' # str |  (opcional)
    type = 'type_example' # str |  (opcional)
    domain = 'domain_example' # str |  (opcional)
    attempt_count_gt = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]