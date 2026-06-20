## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| banEmail | boolean | query | Нет |  |
| banEmailDomain | boolean | query | Нет |  |
| banIP | boolean | query | Нет |  |
| deleteAllUsersComments | boolean | query | Нет |  |
| bannedUntil | string | query | Нет |  |
| isShadowBan | boolean | query | Нет |  |
| updateId | string | query | Нет |  |
| banReason | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Пример

[inline-code-attrs-start title = 'Пример использования post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно; по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (необязательно)
    ban_email_domain = True # bool |  (необязательно)
    ban_ip = True # bool |  (необязательно)
    delete_all_users_comments = True # bool |  (необязательно)
    banned_until = 'banned_until_example' # str |  (необязательно)
    is_shadow_ban = True # bool |  (необязательно)
    update_id = 'update_id_example' # str |  (необязательно)
    ban_reason = 'ban_reason_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]

---