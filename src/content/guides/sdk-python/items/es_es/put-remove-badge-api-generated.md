## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Sí |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de put_remove_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para obtener una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (opcional)
    comment_id = 'comment_id_example' # str |  (opcional)
    broadcast_id = 'broadcast_id_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.put_remove_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]