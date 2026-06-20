## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |

## תגובה

מחזיר: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v1_page_likes.py)

## דוגמה

[inline-code-attrs-start title = 'get_v1_page_likes דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v1_page_likes import GetV1PageLikes
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית ומוגדרת כברירת מחדל ל-https://fastcomments.com
# ראו את configuration.py לרשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# כניסה להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של המחלקה של ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v1_page_likes(tenant_id, url_id)
        print("The response of PublicApi->get_v1_page_likes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v1_page_likes: %s\n" % e)
[inline-code-end]

---