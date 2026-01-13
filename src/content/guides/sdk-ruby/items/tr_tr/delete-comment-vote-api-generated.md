---
## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| commentId | string | path | Evet |  |
| voteId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| broadcastId | string | query | Evet |  |
| editKey | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Dönen değer: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_vote200_response.rb)

## Örnek

[inline-code-attrs-start title = 'delete_comment_vote Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Dize | 
comment_id = 'comment_id_example' # Dize | 
vote_id = 'vote_id_example' # Dize | 
url_id = 'url_id_example' # Dize | 
broadcast_id = 'broadcast_id_example' # Dize | 
opts = {
  edit_key: 'edit_key_example', # Dize | 
  sso: 'sso_example' # Dize | 
}

begin
  
  result = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_comment_vote: #{e}"
end
[inline-code-end]

---