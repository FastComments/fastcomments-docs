## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## Відповідь

Повертає: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове й за замовчуванням — https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче — використайте той,
# який відповідає вашому сценарію автентифікації.

# Налаштування авторизації по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (необов'язково)
    limit = 56 # int |  (необов'язково)
    skip = 56 # int |  (необов'язково)
    as_tree = True # bool |  (необов'язково)
    skip_children = 56 # int |  (необов'язково)
    limit_children = 56 # int |  (необов'язково)
    max_tree_depth = 56 # int |  (необов'язково)
    url_id = 'url_id_example' # str |  (необов'язково)
    user_id = 'user_id_example' # str |  (необов'язково)
    anon_user_id = 'anon_user_id_example' # str |  (необов'язково)
    context_user_id = 'context_user_id_example' # str |  (необов'язково)
    hash_tag = 'hash_tag_example' # str |  (необов'язково)
    parent_id = 'parent_id_example' # str |  (необов'язково)
    direction = client.SortDirections() # SortDirections |  (необов'язково)
    from_date = 56 # int |  (необов'язково)
    to_date = 56 # int |  (необов'язково)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]