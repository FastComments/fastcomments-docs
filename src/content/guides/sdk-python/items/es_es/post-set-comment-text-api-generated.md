## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_text_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de post_set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_text_params import SetCommentTextParams
from client.models.set_comment_text_response import SetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto usa https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    set_comment_text_params = client.SetCommentTextParams() # SetCommentTextParams | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.post_set_comment_text(comment_id, set_comment_text_params, sso=sso)
        print("The response of ModerationApi->post_set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_text: %s\n" % e)
[inline-code-end]