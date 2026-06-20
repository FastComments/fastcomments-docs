## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | integer | query | Не |  |
| limit | integer | query | Не |  |
| skip | integer | query | Не |  |
| asTree | boolean | query | Не |  |
| skipChildren | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |
| contextUserId | string | query | Не |  |
| hashTag | string | query | Не |  |
| parentId | string | query | Не |  |
| direction | string | query | Не |  |
| fromDate | integer | query | Не |  |
| toDate | integer | query | Не |  |

## Одговор

Враћа: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Пример

[inline-code-attrs-start title = 'get_comments Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је на https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Испод су наведени примјери за сваки метод аутентификације, користите примјер који
# одговара вашем случају употребе аутентификације.

# Конфигуришите ауторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (необавезно)
    limit = 56 # int |  (необавезно)
    skip = 56 # int |  (необавезно)
    as_tree = True # bool |  (необавезно)
    skip_children = 56 # int |  (необавезно)
    limit_children = 56 # int |  (необавезно)
    max_tree_depth = 56 # int |  (необавезно)
    url_id = 'url_id_example' # str |  (необавезно)
    user_id = 'user_id_example' # str |  (необавезно)
    anon_user_id = 'anon_user_id_example' # str |  (необавезно)
    context_user_id = 'context_user_id_example' # str |  (необавезно)
    hash_tag = 'hash_tag_example' # str |  (необавезно)
    parent_id = 'parent_id_example' # str |  (необавезно)
    direction = client.SortDirections() # SortDirections |  (необавезно)
    from_date = 56 # int |  (необавезно)
    to_date = 56 # int |  (необавезно)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]