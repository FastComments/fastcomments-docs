## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y, por defecto, es https://fastcomments.com
# Consulta configuration.py para una lista de todos los parámetros de configuración admitidos.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor de la API.
# Se proporcionan ejemplos para cada método de autenticación a continuación; utiliza el ejemplo que
# satisfaga tu caso de uso de autenticación.

# Configurar la autorización por clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomenta abajo para configurar el prefijo (p. ej. Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crea una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (opcional)
    badge_id = 'badge_id_example' # str |  (opcional)
    type = 3.4 # float |  (opcional)
    displayed_on_comments = True # bool |  (opcional)
    limit = 3.4 # float |  (opcional)
    skip = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]