## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| commentIds | string | query | Sí | Una lista separada por comas de identificadores de comentarios. |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/check_blocked_comments_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de checked_comments_for_blocked'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.check_blocked_comments_response import CheckBlockedCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre en un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | Una lista separada por comas de identificadores de comentarios.
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]