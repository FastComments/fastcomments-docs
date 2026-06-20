## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| includeEmail | boolean | query | 否 |  |
| includeIP | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_comment_response.rb)

## 示例

[inline-code-attrs-start title = 'get_moderation_comment 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # 字符串 | 
opts = {
  include_email: true, # 布尔 | 
  include_ip: true, # 布尔 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.get_moderation_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_moderation_comment: #{e}"
end
[inline-code-end]

---