הפעל או השבת התראות עבור דף. כאשר משתמשים מנויים לדף, נוצרות התראות עבור תגובות שורש חדשות, וגם

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |
| url | string | query | כן |  |
| pageTitle | string | query | כן |  |
| subscribedOrUnsubscribed | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_page_subscription_status_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_user_notification_page_subscription_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
url_id = 'url_id_example' # מחרוזת | 
url = 'url_example' # מחרוזת | 
page_title = 'page_title_example' # מחרוזת | 
subscribed_or_unsubscribed = 'subscribe' # מחרוזת | 
opts = {
  sso: 'sso_example' # מחרוזת | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]