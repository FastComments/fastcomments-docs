## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/unblock_success.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-un_block_comment_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  
  result = api_instance.un_block_comment_public(tenant_id, comment_id, public_block_from_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->un_block_comment_public: #{e}"
end
[inline-code-end]

---