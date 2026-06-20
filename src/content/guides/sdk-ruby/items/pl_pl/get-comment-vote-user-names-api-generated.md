## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| commentId | string | path | Tak |  |
| dir | integer | query | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_vote_user_names_success_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comment_vote_user_names'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
dir = 56 # Integer | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_vote_user_names: #{e}"
end
[inline-code-end]

---