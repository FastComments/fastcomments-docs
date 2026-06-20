## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 | 用於判斷目前頁面是否已訂閱。 |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| includeTenantNotifications | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳： [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_my_notifications_response.rb)

## 範例

[inline-code-attrs-start title = 'get_user_notifications 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  url_id: 'url_id_example', # String | 用於判斷目前頁面是否已訂閱。
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