## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-logout_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אינה חובה וברירת המחדל היא https://fastcomments.com
# עיין בקובץ configuration.py לקבלת רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)

    try:
        api_response = api_instance.logout_public()
        print("The response of PublicApi->logout_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->logout_public: %s\n" % e)
[inline-code-end]