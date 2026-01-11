## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_notifications200_response.rb)

## 示例

[inline-code-attrs-start title = 'get_user_notifications 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
opts = {
  page_size: 56, # 整数 | 
  after_id: 'after_id_example', # 字符串 | 
  include_context: true, # 布尔值 | 
  after_created_at: 789, # 整数 | 
  unread_only: true, # 布尔值 | 
  dm_only: true, # 布尔值 | 
  no_dm: true, # 布尔值 | 
  include_translations: true, # 布尔值 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.get_user_notifications(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notifications: #{e}"
end
[inline-code-end]

---