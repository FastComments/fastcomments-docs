---
## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| broadcastId | string | query | Sí |  |
| sessionId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_comment_public200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de create_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comment_data import CommentData
from client.models.create_comment_public200_response import CreateCommentPublic200Response
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
    broadcast_id = 'broadcast_id_example' # str | 
    comment_data = client.CommentData() # CommentData | 
    session_id = 'session_id_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.create_comment_public(tenant_id, url_id, broadcast_id, comment_data, session_id=session_id, sso=sso)
        print("The response of PublicApi->create_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_comment_public: %s\n" % e)
[inline-code-end]

---