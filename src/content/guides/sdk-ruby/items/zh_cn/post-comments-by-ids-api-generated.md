## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 响应

返回：[`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_child_comments_response.rb)

## 示例

[inline-code-attrs-start title = 'post_comments_by_ids 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comments_by_ids_params = FastCommentsClient::CommentsByIdsParams.new({ids: ['ids_example']}) # CommentsByIdsParams | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_comments_by_ids(comments_by_ids_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_comments_by_ids: #{e}"
end
[inline-code-end]

---