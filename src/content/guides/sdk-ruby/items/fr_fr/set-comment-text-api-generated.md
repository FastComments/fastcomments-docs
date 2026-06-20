## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| broadcastId | string | query | Oui |  |
| editKey | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_a_p_i_set_comment_text_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de set_comment_text'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
comment_id = 'comment_id_example' # Chaîne | 
broadcast_id = 'broadcast_id_example' # Chaîne | 
comment_text_update_request = FastCommentsClient::CommentTextUpdateRequest.new({comment: 'comment_example'}) # CommentTextUpdateRequest | 
opts = {
  edit_key: 'edit_key_example', # Chaîne | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->set_comment_text: #{e}"
end
[inline-code-end]