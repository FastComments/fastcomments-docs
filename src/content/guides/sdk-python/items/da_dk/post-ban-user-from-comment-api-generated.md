## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nej |  |
| banEmailDomain | boolean | query | Nej |  |
| banIP | boolean | query | Nej |  |
| deleteAllUsersComments | boolean | query | Nej |  |
| bannedUntil | string | query | Nej |  |
| isShadowBan | boolean | query | Nej |  |
| updateId | string | query | Nej |  |
| banReason | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Eksempel

[inline-code-attrs-start title = 'post_ban_user_from_comment Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
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