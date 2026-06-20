---
## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| sso | string | query | No |  |

## Respuesta

Devuelve: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_moderate_get_user_ban_preferences_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_user_ban_preference'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_moderate_get_user_ban_preferences_response import APIModerateGetUserBanPreferencesResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y, por defecto, es https://fastcomments.com
# Consulta configuration.py para una lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Crea una instancia de la clase API
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_user_ban_preference(sso=sso)
        print("The response of ModerationApi->get_user_ban_preference:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_ban_preference: %s\n" % e)
[inline-code-end]

---