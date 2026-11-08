## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| namespace | string | path | हाँ |  |
| component | string | path | हाँ |  |
| locale | string | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_translations उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetTranslationsOptions
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# होस्ट परिभाषित करना वैकल्पिक है और डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के एक इंस्टेंस के साथ कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएँ
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (वैकल्पिक)
    use_full_translation_ids = True # bool |  (वैकल्पिक)

    try:
        api_response = api_instance.get_translations(namespace, component, GetTranslationsOptions(locale=locale, use_full_translation_ids=use_full_translation_ids))
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]