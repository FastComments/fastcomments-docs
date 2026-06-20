---
페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새 루트 댓글에 대해 알림이 생성되고, 또한

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 예 |  |
| url | string | query | 예 |  |
| pageTitle | string | query | 예 |  |
| subscribedOrUnsubscribed | string | path | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_page_subscription_status_response.rb)

## 예제

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
url_id = 'url_id_example' # 문자열 | 
url = 'url_example' # 문자열 | 
page_title = 'page_title_example' # 문자열 | 
subscribed_or_unsubscribed = 'subscribe' # 문자열 | 
opts = {
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]

---