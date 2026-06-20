## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| largeInternalURLSanitized | string | query | Yes |  |

## Respuesta

Devuelve: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/gif_get_large_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_gif_large'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.gif_get_large_response import GifGetLargeResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    large_internal_url_sanitized = 'large_internal_url_sanitized_example' # str | 

    try:
        api_response = api_instance.get_gif_large(tenant_id, large_internal_url_sanitized)
        print("The response of PublicApi->get_gif_large:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gif_large: %s\n" % e)
[inline-code-end]

---