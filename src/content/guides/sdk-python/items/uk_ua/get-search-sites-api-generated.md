## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_site_search_response.py)

## Приклад

[inline-code-attrs-start title = 'get_search_sites Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_site_search_response import ModerationSiteSearchResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням — https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Використовуйте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.get_search_sites(value=value, sso=sso)
        print("The response of ModerationApi->get_search_sites:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_sites: %s\n" % e)
[inline-code-end]

---