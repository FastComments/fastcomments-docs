## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | No |  |

## Respuesta

Devuelve: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tags_bulk200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de add_hash_tags_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tags_bulk200_response import AddHashTagsBulk200Response
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor de la API.
# Se proporcionan ejemplos para cada método de autenticación a continuación; use el que
# satisfaga su caso de uso de autenticación.

# Configurar la autorización por clave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar el prefijo (p. ej., Bearer) para la clave de API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrar en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (opcional)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (opcional)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]