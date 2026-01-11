## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| domainToUpdate | string | path | Sí |  |

## Respuesta

Devuelve: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_config200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de patch_domain_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_config200_response import GetDomainConfig200Response
from client.models.patch_domain_config_params import PatchDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulta configuration.py para obtener la lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación más abajo, usa el ejemplo que
# satisfaga tu caso de uso de autenticación.

# Configurar la autorización por clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomenta lo de abajo para configurar el prefijo (p. ej. Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Crea una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    patch_domain_config_params = client.PatchDomainConfigParams() # PatchDomainConfigParams | 

    try:
        api_response = api_instance.patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)
        print("The response of DefaultApi->patch_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_domain_config: %s\n" % e)
[inline-code-end]