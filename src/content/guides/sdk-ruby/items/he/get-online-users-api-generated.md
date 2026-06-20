צופים שמחוברים כעת לעמוד: אנשים שה-session שלהם ב-websocket מנוי לעמוד כרגע.
מחזיר anonCount + totalCount (מנויים ברחבי החדר, כולל צופים אנונימיים שאותם אנו לא מפרטים).

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה URL של העמוד (מנוקה בצד השרת). |
| afterName | string | query | לא | Cursor: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | לא | שובר שוויון עבור Cursor: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שוויון בשמות לא יגרמו להשמטת רשומות. |

## תגובה

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## דוגמה

[inline-code-attrs-start title = 'get_online_users Example'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | מזהה URL של העמוד (מנוקה בצד השרת).
opts = {
  after_name: 'after_name_example', # String | Cursor: העבר את nextAfterName מהתגובה הקודמת.
  after_user_id: 'after_user_id_example' # String | שובר שוויון עבור Cursor: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שוויון בשמות לא יגרמו להשמטת רשומות.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---