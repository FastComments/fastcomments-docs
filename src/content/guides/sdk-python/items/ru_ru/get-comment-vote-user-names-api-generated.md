## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| dir | integer | query | Yes |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_vote_user_names_success_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comment_vote_user_names'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_vote_user_names_success_response import GetCommentVoteUserNamesSuccessResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войти в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создать экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    dir = 56 # int | 
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, sso=sso)
        print("The response of PublicApi->get_comment_vote_user_names:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_vote_user_names: %s\n" % e)
[inline-code-end]

---