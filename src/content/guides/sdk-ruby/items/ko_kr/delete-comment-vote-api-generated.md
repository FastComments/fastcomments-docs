## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| commentId | string | 경로 | 예 |  |
| voteId | string | 경로 | 예 |  |
| urlId | string | 쿼리 | 예 |  |
| broadcastId | string | 쿼리 | 예 |  |
| editKey | string | 쿼리 | 아니요 |  |
| sso | string | 쿼리 | 아니요 |  |

## 응답

반환: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_vote200_response.rb)

## 예제

[inline-code-attrs-start title = 'delete_comment_vote 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
comment_id = 'comment_id_example' # 문자열 | 
vote_id = 'vote_id_example' # 문자열 | 
url_id = 'url_id_example' # 문자열 | 
broadcast_id = 'broadcast_id_example' # 문자열 | 
opts = {
  edit_key: 'edit_key_example', # 문자열 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_comment_vote: #{e}"
end
[inline-code-end]