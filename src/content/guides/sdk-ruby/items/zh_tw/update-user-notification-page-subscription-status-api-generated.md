啟用或停用頁面通知。當使用者訂閱某頁面時，系統會為新的頂層留言建立通知，並且還會

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| url | string | query | Yes |  |
| pageTitle | string | query | Yes |  |
| subscribedOrUnsubscribed | string | path | Yes |  |
| sso | string | query | No |  |

## Response

回傳： [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## Example

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
url_id = 'url_id_example' # 字串 | 
url = 'url_example' # 字串 | 
page_title = 'page_title_example' # 字串 | 
subscribed_or_unsubscribed = 'subscribe' # 字串 | 
opts = {
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]