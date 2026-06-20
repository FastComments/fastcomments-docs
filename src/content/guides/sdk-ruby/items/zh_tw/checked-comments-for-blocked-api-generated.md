---
## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | 以逗號分隔的評論 ID 列表。 |
| sso | string | query | No |  |

## 回應

回傳: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/check_blocked_comments_response.rb)

## 範例

[inline-code-attrs-start title = 'checked_comments_for_blocked 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_ids = 'comment_ids_example' # String | 以逗號分隔的評論 ID 列表。
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->checked_comments_for_blocked: #{e}"
end
[inline-code-end]

---