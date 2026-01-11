## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| isFlagged | boolean | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## 範例

[inline-code-attrs-start title = 'flag_comment_public 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
comment_id = 'comment_id_example' # 字串 | 
is_flagged = true # 布林值 | 
opts = {
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.flag_comment_public(tenant_id, comment_id, is_flagged, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->flag_comment_public: #{e}"
end
[inline-code-end]