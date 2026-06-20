## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Esempio

[inline-code-attrs-start title = 'Esempio post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedere configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (opzionale)
    ban_email_domain = True # bool |  (opzionale)
    ban_ip = True # bool |  (opzionale)
    delete_all_users_comments = True # bool |  (opzionale)
    banned_until = 'banned_until_example' # str |  (opzionale)
    is_shadow_ban = True # bool |  (opzionale)
    update_id = 'update_id_example' # str |  (opzionale)
    ban_reason = 'ban_reason_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]