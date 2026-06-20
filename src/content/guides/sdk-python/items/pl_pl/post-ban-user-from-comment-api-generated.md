## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| banEmail | boolean | query | Nie |  |
| banEmailDomain | boolean | query | Nie |  |
| banIP | boolean | query | Nie |  |
| deleteAllUsersComments | boolean | query | Nie |  |
| bannedUntil | string | query | Nie |  |
| isShadowBan | boolean | query | Nie |  |
| updateId | string | query | Nie |  |
| banReason | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Przykład

[inline-code-attrs-start title = 'Przykład post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (opcjonalne)
    ban_email_domain = True # bool |  (opcjonalne)
    ban_ip = True # bool |  (opcjonalne)
    delete_all_users_comments = True # bool |  (opcjonalne)
    banned_until = 'banned_until_example' # str |  (opcjonalne)
    is_shadow_ban = True # bool |  (opcjonalne)
    update_id = 'update_id_example' # str |  (opcjonalne)
    ban_reason = 'ban_reason_example' # str |  (opcjonalne)
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]