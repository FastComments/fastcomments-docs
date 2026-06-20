## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| afterId | string | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| sso | string | query | No |  |

## 回應

回傳: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/reset_user_notifications_response.rb)

## 範例

[inline-code-attrs-start title = 'reset_user_notifications 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
opts = {
  after_id: 'after_id_example', # 字串 | 
  after_created_at: 789, # 整數 | 
  unread_only: true, # 布林值 | 
  dm_only: true, # 布林值 | 
  no_dm: true, # 布林值 | 
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.reset_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->reset_user_notifications: #{e}"
end
[inline-code-end]