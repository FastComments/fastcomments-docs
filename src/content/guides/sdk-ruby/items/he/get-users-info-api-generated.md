מידע משתמשים באצווה עבור טננט. בהינתן userIds, מחזיר מידע להצגה מ-User / SSOUser.
משמש את ווידג'ט ההערות להעשיר משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
אין הקשר של דף: הפרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds מופרדים בפסיקים. |

## תגובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | Comma-delimited userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---