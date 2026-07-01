## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| namespace | string | yol | Evet |  |
| component | string | yol | Evet |  |
| locale | string | sorgu | Hayır |  |
| useFullTranslationIds | boolean | sorgu | Hayır |  |

## Yanıt

Döndürür: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## Örnek

[inline-code-attrs-start title = 'get_translations Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetTranslationsOptions
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# Host'i tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine ayarlanır
# configuration.py dosyasında desteklenen tüm yapılandırma parametrelerinin bir listesini bulabilirsiniz.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (optional)
    use_full_translation_ids = True # bool |  (optional)

    try:
        api_response = api_instance.get_translations(namespace, component, GetTranslationsOptions(locale=locale, use_full_translation_ids=use_full_translation_ids))
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]