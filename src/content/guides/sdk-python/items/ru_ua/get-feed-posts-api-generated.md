req
tenantId
afterId

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| afterId | string | query | Ні |  |
| limit | integer | query | Ні |  |
| tags | array | query | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_feed_posts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetFeedPostsOptions
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов’язковим і за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки API сервера.
# Приклади для кожного методу автентифікації надані нижче, використайте приклад,
# який відповідає вашому випадку використання.

# Налаштуйте авторизацію за ключем API: api_key
# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (необов'язковий)
    limit = 56 # int |  (необов'язковий)
    tags = ['tags_example'] # List[str] |  (необов'язковий)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, GetFeedPostsOptions(after_id=after_id, limit=limit, tags=tags))
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]