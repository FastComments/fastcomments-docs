## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|----------|------------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de update_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_email_template_body import UpdateEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación a continuación; use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_email_template_body = client.UpdateEmailTemplateBody() # UpdateEmailTemplateBody | 

    try:
        api_response = api_instance.update_email_template(tenant_id, id, update_email_template_body)
        print("The response of DefaultApi->update_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_email_template: %s\n" % e)
[inline-code-end]