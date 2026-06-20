## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| approved | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_approved_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de post_set_comment_approval_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_approved_response import SetCommentApprovedResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto utiliza https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración soportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ingrese a un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    approved = True # bool |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.post_set_comment_approval_status(comment_id, approved=approved, sso=sso)
        print("The response of ModerationApi->post_set_comment_approval_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_approval_status: %s\n" % e)
[inline-code-end]