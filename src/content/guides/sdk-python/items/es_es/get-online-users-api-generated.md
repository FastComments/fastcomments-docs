Espectadores actualmente conectados a una página: personas cuya sesión websocket está suscrita a la página en este momento.
Devuelve anonCount + totalCount (suscriptores de toda la sala, incluidos los espectadores anónimos que no enumeramos).

## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador de la URL de la página (limpiado en el servidor). |
| afterName | string | query | No | Cursor: pase nextAfterName de la respuesta anterior. |
| afterUserId | string | query | No | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no descarten entradas. |

## Respuesta

Devuelve: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identificador de URL de la página (limpiado en el servidor).
    after_name = 'after_name_example' # str | Cursor: pase nextAfterName de la respuesta anterior. (opcional)
    after_user_id = 'after_user_id_example' # str | Desempate del cursor: pase nextAfterUserId de la respuesta anterior. Requerido cuando afterName está establecido para que los empates por nombre no descarten entradas. (opcional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]

---