特定のコメントに対する通知を有効または無効にします。

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| notificationId | string | path | はい |  |
| optedInOrOut | string | path | はい |  |
| commentId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status200_response.rb)

## 例

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
notification_id = 'notification_id_example' # 文字列 | 
opted_in_or_out = 'in' # 文字列 | 
comment_id = 'comment_id_example' # 文字列 | 
opts = {
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_comment_subscription_status: #{e}"
end
[inline-code-end]

---