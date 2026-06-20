## 参数

| 名称 | 类型 | 位置 | 必填 | 说明 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/post_remove_comment_response.rb)

## 示例

[inline-code-attrs-start title = 'post_remove_comment 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # 字符串 | 
opts = {
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.post_remove_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_remove_comment: #{e}"
end
[inline-code-end]

---