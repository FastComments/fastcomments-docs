## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sorts | string | query | לא |  |
| sso | string | query | לא |  |

## Response

מחזיר: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## Example

[inline-code-attrs-start title = 'post_api_export דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostApiExportOptions
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית והמוגדרת כברירת מחדל ל‑https://fastcomments.com
# ראו configuration.py לרשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסת הקשר עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה‑API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (אופציונלי)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (אופציונלי)
    filters = 'filters_example' # str |  (אופציונלי)
    search_filters = 'search_filters_example' # str |  (אופציונלי)
    sorts = 'sorts_example' # str |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.post_api_export(tenant_id, PostApiExportOptions(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso))
        print("תגובת ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("חריגה בעת קריאה ל‑ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]