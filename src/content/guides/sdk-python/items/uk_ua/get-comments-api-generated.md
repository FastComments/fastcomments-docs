## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| page | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| skip | integer | query | Ні |  |
| asTree | boolean | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| urlId | string | query | Ні |  |
| userId | string | query | Ні |  |
| anonUserId | string | query | Ні |  |
| contextUserId | string | query | Ні |  |
| hashTag | string | query | Ні |  |
| parentId | string | query | Ні |  |
| direction | string | query | Ні |  |

## Відповідь

Повертає: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## Приклад

[inline-code-attrs-start title = 'get_comments Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове, за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Нижче наведено приклади для кожного методу автентифікації, використайте приклад, який
# відповідає вашому випадку використання автентифікації.

# Налаштуйте авторизацію за допомогою API ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст з екземпляром клієнта API
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

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]