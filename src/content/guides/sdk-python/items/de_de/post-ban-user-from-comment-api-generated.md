## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nein |  |
| banEmailDomain | boolean | query | Nein |  |
| banIP | boolean | query | Nein |  |
| deleteAllUsersComments | boolean | query | Nein |  |
| bannedUntil | string | query | Nein |  |
| isShadowBan | boolean | query | Nein |  |
| updateId | string | query | Nein |  |
| banReason | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Beispiel

[inline-code-attrs-start title = 'post_ban_user_from_comment Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients verwenden
with client.ApiClient(configuration) as api_client:
    # Erstellen einer Instanz der API-Klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (optional)
    ban_email_domain = True # bool |  (optional)
    ban_ip = True # bool |  (optional)
    delete_all_users_comments = True # bool |  (optional)
    banned_until = 'banned_until_example' # str |  (optional)
    is_shadow_ban = True # bool |  (optional)
    update_id = 'update_id_example' # str |  (optional)
    ban_reason = 'ban_reason_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]