啟用或停用特定留言的通知。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| notificationId | string | path | 是 |  |
| optedInOrOut | string | path | 是 |  |
| commentId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## 範例

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
notification_id = 'notification_id_example' # 字串 | 
opted_in_or_out = 'in' # 字串 | 
comment_id = 'comment_id_example' # 字串 | 
opts = {
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_comment_subscription_status: #{e}"
end
[inline-code-end]

---