## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| dir | integer | query | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_vote_user_names200_response.py)

## Пример

[inline-code-attrs-start title = 'get_comment_vote_user_names Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_vote_user_names200_response import GetCommentVoteUserNames200Response
from client.rest import ApiException
from pprint import pprint

# Подешавање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    dir = 56 # int | 
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, sso=sso)
        print("The response of PublicApi->get_comment_vote_user_names:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_vote_user_names: %s\n" % e)
[inline-code-end]

---