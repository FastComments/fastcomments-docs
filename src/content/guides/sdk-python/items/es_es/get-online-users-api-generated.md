Visores actualmente en línea de una página: personas cuya sesión websocket está suscrita a la página en este momento.  
Devuelve anonCount + totalCount (suscriptores de toda la sala, incluidos los visitantes anónimos que no enumeramos).

## Parameters

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí | Identificador de URL de la página (limpiado del lado del servidor). |
| afterName | string | query | No | Cursor: pasar nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate de cursor: pasar nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas. |

## Response

Devuelve: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Ver configuration.py para una lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificador de URL de la página (limpiado del lado del servidor).
    after_name = 'after_name_example' # str | Cursor: pasar nextAfterName de la respuesta anterior. (opcional)
    after_user_id = 'after_user_id_example' # str | Desempate de cursor: pasar nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates de nombre no eliminen entradas. (opcional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]

---