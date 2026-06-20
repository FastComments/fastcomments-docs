## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_search_suggest'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_search_suggest(text_search=text_search, sso=sso)
        print("The response of ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]

---