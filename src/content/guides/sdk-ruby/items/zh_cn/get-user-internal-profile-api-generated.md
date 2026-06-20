---
## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| commentId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_internal_profile_response.rb)

## 示例

[inline-code-attrs-start title = 'get_user_internal_profile 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  comment_id: 'comment_id_example', # 字符串 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.get_user_internal_profile(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_user_internal_profile: #{e}"
end
[inline-code-end]

---