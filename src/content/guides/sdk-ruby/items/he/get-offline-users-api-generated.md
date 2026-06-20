מצביעים שניקבו בעבר בעמוד ואינם מקוונים כרגע. ממוינים לפי displayName.
השתמש בזה לאחר שתרוקנו את /users/online כדי להציג מדור 'חברים'.
דפדוף מבוסס סמן על commenterName: השרת מהלך את האינדקס החלקי {tenantId, urlId, commenterName} מהafterName והלאה באמצעות $gt, ללא עלות $skip.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה כתובת ה-URL של הדף (מנוקה בצד השרת). |
| afterName | string | query | לא | סמן: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | לא | שובר שוויון של הסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי ששוויון בשמות לא יגרום להחמצת רשומות. |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | מזהה כתובת ה-URL של הדף (מנוקה בצד השרת).
opts = {
  after_name: 'after_name_example', # String | סמן: העבר את nextAfterName מהתגובה הקודמת.
  after_user_id: 'after_user_id_example' # String | שובר שוויון של הסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי ששוויון בשמות לא יגרום להחמצת רשומות.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---