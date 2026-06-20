## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| afterId | string | query | 아니오 |  |
| afterCreatedAt | integer | query | 아니오 |  |
| unreadOnly | boolean | query | 아니오 |  |
| dmOnly | boolean | query | 아니오 |  |
| noDm | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/reset_user_notifications_response.rb)

## 예제

[inline-code-attrs-start title = 'reset_user_notifications 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
opts = {
  after_id: 'after_id_example', # 문자열 | 
  after_created_at: 789, # 정수 | 
  unread_only: true, # 부울 | 
  dm_only: true, # 부울 | 
  no_dm: true, # 부울 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.reset_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->reset_user_notifications: #{e}"
end
[inline-code-end]