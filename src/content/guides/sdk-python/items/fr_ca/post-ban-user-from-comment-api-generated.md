## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Renvoie : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (facultatif)
    ban_email_domain = True # bool |  (facultatif)
    ban_ip = True # bool |  (facultatif)
    delete_all_users_comments = True # bool |  (facultatif)
    banned_until = 'banned_until_example' # str |  (facultatif)
    is_shadow_ban = True # bool |  (facultatif)
    update_id = 'update_id_example' # str |  (facultatif)
    ban_reason = 'ban_reason_example' # str |  (facultatif)
    sso = 'sso_example' # str |  (facultatif)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]