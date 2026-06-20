启用或禁用对特定评论的通知。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| notificationId | string | path | 是 |  |
| optedInOrOut | string | path | 是 |  |
| commentId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_comment_subscription_status_response.rb)

## 示例

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
notification_id = 'notification_id_example' # String | 
opted_in_or_out = 'in' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_comment_subscription_status: #{e}"
end
[inline-code-end]