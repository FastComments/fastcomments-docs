## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (opcional)
    ban_email_domain = True # bool |  (opcional)
    ban_ip = True # bool |  (opcional)
    delete_all_users_comments = True # bool |  (opcional)
    banned_until = 'banned_until_example' # str |  (opcional)
    is_shadow_ban = True # bool |  (opcional)
    update_id = 'update_id_example' # str |  (opcional)
    ban_reason = 'ban_reason_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]