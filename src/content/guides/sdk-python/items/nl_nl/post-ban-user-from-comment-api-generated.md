## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nee |  |
| banEmailDomain | boolean | query | Nee |  |
| banIP | boolean | query | Nee |  |
| deleteAllUsersComments | boolean | query | Nee |  |
| bannedUntil | string | query | Nee |  |
| isShadowBan | boolean | query | Nee |  |
| updateId | string | query | Nee |  |
| banReason | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Voorbeeld

[inline-code-attrs-start title = 'post_ban_user_from_comment Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (optioneel)
    ban_email_domain = True # bool |  (optioneel)
    ban_ip = True # bool |  (optioneel)
    delete_all_users_comments = True # bool |  (optioneel)
    banned_until = 'banned_until_example' # str |  (optioneel)
    is_shadow_ban = True # bool |  (optioneel)
    update_id = 'update_id_example' # str |  (optioneel)
    ban_reason = 'ban_reason_example' # str |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]