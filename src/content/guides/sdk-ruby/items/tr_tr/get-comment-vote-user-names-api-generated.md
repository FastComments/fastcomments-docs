## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | dize | yol | Evet |  |
| commentId | dize | yol | Evet |  |
| dir | tamsayı | sorgu | Evet |  |
| sso | dize | sorgu | Hayır |  |

## Yanıt

Döndürür: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_vote_user_names200_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_comment_vote_user_names Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Dize | 
comment_id = 'comment_id_example' # Dize | 
dir = 56 # Tamsayı | 
opts = {
  sso: 'sso_example' # Dize | 
}

begin
  
  result = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_vote_user_names: #{e}"
end
[inline-code-end]