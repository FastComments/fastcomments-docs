## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|--------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_get_comment_text_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentTextOptions
from client.models.public_api_get_comment_text_response import PublicAPIGetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Ver configuration.py para una lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrar en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_comment_text(tenant_id, comment_id, GetCommentTextOptions(edit_key=edit_key, sso=sso))
        print("The response of PublicApi->get_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]