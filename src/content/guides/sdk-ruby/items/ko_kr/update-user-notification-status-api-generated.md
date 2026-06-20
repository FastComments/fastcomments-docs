## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| notificationId | string | path | 예 |  |
| newStatus | string | path | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_user_notification_status_response.rb)

## 예제

[inline-code-attrs-start title = 'update_user_notification_status 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
notification_id = 'notification_id_example' # 문자열 | 
new_status = 'read' # 문자열 | 
opts = {
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_status: #{e}"
end
[inline-code-end]