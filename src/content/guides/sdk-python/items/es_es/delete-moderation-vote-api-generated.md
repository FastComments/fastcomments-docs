## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| voteId | string | path | Sí |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_moderation_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abrir un contexto con una instancia del cliente API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    vote_id = 'vote_id_example' # str | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.delete_moderation_vote(comment_id, vote_id, sso=sso)
        print("The response of ModerationApi->delete_moderation_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->delete_moderation_vote: %s\n" % e)
[inline-code-end]