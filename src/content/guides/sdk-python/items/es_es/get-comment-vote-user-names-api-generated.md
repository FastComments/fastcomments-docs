## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| commentId | string | path | Sí |  |
| dir | integer | query | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_vote_user_names_success_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_comment_vote_user_names'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_vote_user_names_success_response import GetCommentVoteUserNamesSuccessResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para obtener la lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    dir = 56 # int | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, sso=sso)
        print("The response of PublicApi->get_comment_vote_user_names:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_vote_user_names: %s\n" % e)
[inline-code-end]