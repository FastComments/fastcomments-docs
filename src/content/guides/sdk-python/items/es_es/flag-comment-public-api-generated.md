## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentId | string | path | Sí |  |
| isFlagged | boolean | query | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de flag_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    is_flagged = True # bool | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.flag_comment_public(tenant_id, comment_id, is_flagged, sso=sso)
        print("The response of PublicApi->flag_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->flag_comment_public: %s\n" % e)
[inline-code-end]