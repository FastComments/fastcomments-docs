租户的批量用户信息。给定 userIds，返回来自 User / SSOUser 的显示信息。
由评论小部件使用，用于丰富通过在线状态事件刚刚出现的用户。
没有页面上下文：隐私被统一强制（私密资料被屏蔽）。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 逗号分隔的 userIds。 |

## 响应

返回：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## 示例

[inline-code-attrs-start title = 'get_users_info 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | 逗号分隔的 userIds。

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]