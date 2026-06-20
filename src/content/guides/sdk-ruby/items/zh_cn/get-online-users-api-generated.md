当前在线的页面查看者：其 websocket 会话当前已订阅该页面的人。
返回 anonCount + totalCount（房间范围的订阅者，包括我们不逐一列举的匿名查看者）。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（在服务器端清理）。 |
| afterName | string | query | 否 | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标的决胜项：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时需要提供，以免同名导致条目丢失。 |

## 响应

返回: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## 示例

[inline-code-attrs-start title = 'get_online_users 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字符串 | 
url_id = 'url_id_example' # 字符串 | 页面 URL 标识符（在服务器端清理）。
opts = {
  after_name: 'after_name_example', # 字符串 | 游标：传入上一次响应中的 nextAfterName。
  after_user_id: 'after_user_id_example' # 字符串 | 游标的决胜项：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时需要提供，以免同名导致条目丢失。
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]