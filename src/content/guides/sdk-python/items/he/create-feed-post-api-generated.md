## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| broadcastId | string | query | לא |  |
| isLive | boolean | query | לא |  |
| doSpamCheck | boolean | query | לא |  |
| skipDupCheck | boolean | query | לא |  |

## תגובה

מחזיר: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post200_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-create_feed_post'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post200_response import CreateFeedPost200Response
from client.models.create_feed_post_params import CreateFeedPostParams
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית וברירת המחדל היא https://fastcomments.com
# ראה את configuration.py עבור רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# הלקוח חייב להגדיר את פרמטרי האימות וההרשאה
# בהתאם למדיניות האבטחה של שרת ה-API.
# דוגמאות לכל שיטת אימות מסופקות למטה, השתמש בדוגמה ש
# מספקת את מקרה השימוש שלך לאימות.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (אופציונלי)
    is_live = True # bool |  (אופציונלי)
    do_spam_check = True # bool |  (אופציונלי)
    skip_dup_check = True # bool |  (אופציונלי)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check)
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]