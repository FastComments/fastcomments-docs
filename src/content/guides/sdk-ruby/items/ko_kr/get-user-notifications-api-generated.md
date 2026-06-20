## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 아니요 | 현재 페이지가 구독되어 있는지 확인하는 데 사용됩니다. |
| pageSize | integer | query | 아니요 |  |
| afterId | string | query | 아니요 |  |
| includeContext | boolean | query | 아니요 |  |
| afterCreatedAt | integer | query | 아니요 |  |
| unreadOnly | boolean | query | 아니요 |  |
| dmOnly | boolean | query | 아니요 |  |
| noDm | boolean | query | 아니요 |  |
| includeTranslations | boolean | query | 아니요 |  |
| includeTenantNotifications | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_my_notifications_response.rb)

## 예제

[inline-code-attrs-start title = 'get_user_notifications 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  url_id: 'url_id_example', # String | 현재 페이지가 구독되어 있는지 확인하는 데 사용됩니다.
  page_size: 56, # Integer | 
  after_id: 'after_id_example', # String | 
  include_context: true, # Boolean | 
  after_created_at: 789, # Integer | 
  unread_only: true, # Boolean | 
  dm_only: true, # Boolean | 
  no_dm: true, # Boolean | 
  include_translations: true, # Boolean | 
  include_tenant_notifications: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notifications: #{e}"
end
[inline-code-end]