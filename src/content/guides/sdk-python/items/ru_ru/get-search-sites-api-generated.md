## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| value | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_site_search_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_search_sites'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_site_search_response import ModerationSiteSearchResponse
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_search_sites(value=value, sso=sso)
        print("The response of ModerationApi->get_search_sites:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_sites: %s\n" % e)
[inline-code-end]