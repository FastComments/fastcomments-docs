---
在页面上过去发表评论但当前不在线的用户。按 displayName 排序。
在用尽 /users/online 之后使用此方法以呈现 "Members" 部分。
对 commenterName 使用游标分页：服务器从 afterName 向前通过 $gt 遍历部分索引 {tenantId, urlId, commenterName}，无需 $skip 成本。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 页面 URL 标识符（服务器端清理）。 |
| afterName | string | query | 否 | 游标：传递上一个响应中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标决胜项：传递上一个响应中的 nextAfterUserId。当设置了 afterName 时需要，以防同名导致条目被丢弃。 |

## Response

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Example

[inline-code-attrs-start title = 'get_offline_users 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 页面 URL 标识符（服务器端清理）。
opts = {
  after_name: 'after_name_example', # String | 游标：传递上一个响应中的 nextAfterName。
  after_user_id: 'after_user_id_example' # String | 游标决胜项：传递上一个响应中的 nextAfterUserId。 当设置了 afterName 时需要，以防同名导致条目丢失。
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---