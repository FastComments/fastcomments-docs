## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| namespace | string | path | כן |  |
| component | string | path | כן |  |
| locale | string | query | לא |  |
| useFullTranslationIds | boolean | query | לא |  |

## תגובה

מחזיר: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_translations'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית וברירת המחדל היא https://fastcomments.com
# ראו את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו להקשר עם מופע של ה-API client
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (אופציונלי)
    use_full_translation_ids = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.get_translations(namespace, component, locale=locale, use_full_translation_ids=use_full_translation_ids)
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]

---