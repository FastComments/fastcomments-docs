## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Да |  |
| component | string | path | Да |  |
| locale | string | query | Нет |  |
| useFullTranslationIds | boolean | query | Нет |  |

## Ответ

Возвращает: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_translations'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (необязательно)
    use_full_translation_ids = True # bool |  (необязательно)

    try:
        api_response = api_instance.get_translations(namespace, component, locale=locale, use_full_translation_ids=use_full_translation_ids)
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]