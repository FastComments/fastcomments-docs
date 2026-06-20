啟用或停用頁面的通知。當使用者訂閱頁面時，會為新的根留言建立通知，並且也

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |
| url | string | query | 是 |  |
| pageTitle | string | query | 是 |  |
| subscribedOrUnsubscribed | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_page_subscription_status_response.rb)

## 範例

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
url = 'url_example' # String | 
page_title = 'page_title_example' # String | 
subscribed_or_unsubscribed = 'subscribe' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]