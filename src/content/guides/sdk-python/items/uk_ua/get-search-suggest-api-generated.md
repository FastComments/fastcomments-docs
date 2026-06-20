## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| text-search | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_search_suggest'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# Визначення параметра host необов'язкове та за замовчуванням дорівнює https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_search_suggest(text_search=text_search, sso=sso)
        print("The response of ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]