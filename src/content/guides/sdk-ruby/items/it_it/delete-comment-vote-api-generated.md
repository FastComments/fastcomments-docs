## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| voteId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| broadcastId | string | query | Sì |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_vote200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_comment_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Stringa | 
comment_id = 'comment_id_example' # Stringa | 
vote_id = 'vote_id_example' # Stringa | 
url_id = 'url_id_example' # Stringa | 
broadcast_id = 'broadcast_id_example' # Stringa | 
opts = {
  edit_key: 'edit_key_example', # Stringa | 
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_comment_vote: #{e}"
end
[inline-code-end]

---