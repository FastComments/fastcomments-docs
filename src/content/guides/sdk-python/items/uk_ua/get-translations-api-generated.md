## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| namespace | string | шлях | Так |  |
| component | string | шлях | Так |  |
| locale | string | параметр запиту | Ні |  |
| useFullTranslationIds | boolean | параметр запиту | Ні |  |

## Відповідь

Повертає: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_translations'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту необов'язкове та за замовчуванням використовує https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Використайте контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (необов'язково)
    use_full_translation_ids = True # bool |  (необов'язково)

    try:
        api_response = api_instance.get_translations(namespace, component, locale=locale, use_full_translation_ids=use_full_translation_ids)
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]

---