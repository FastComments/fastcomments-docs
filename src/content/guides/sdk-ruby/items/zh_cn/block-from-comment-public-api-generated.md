## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## 响应

返回: [`BlockSuccess`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/block_success.rb)

## 示例

[inline-code-attrs-start title = 'block_from_comment_public 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
public_block_from_comment_params = FastCommentsClient::PublicBlockFromCommentParams.new({comment_ids: ['comment_ids_example']}) # PublicBlockFromCommentParams | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->block_from_comment_public: #{e}"
end
[inline-code-end]

---