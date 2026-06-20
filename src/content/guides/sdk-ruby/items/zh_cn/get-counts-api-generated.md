---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 响应

返回: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_banned_users_count_response.rb)

## 示例

[inline-code-attrs-start title = 'get_counts 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.get_counts(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_counts: #{e}"
end
[inline-code-end]

---