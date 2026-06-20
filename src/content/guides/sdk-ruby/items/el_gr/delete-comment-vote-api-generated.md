## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| voteId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| broadcastId | string | query | Ναι |  |
| editKey | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_delete_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα delete_comment_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
comment_id = 'comment_id_example' # Συμβολοσειρά | 
vote_id = 'vote_id_example' # Συμβολοσειρά | 
url_id = 'url_id_example' # Συμβολοσειρά | 
broadcast_id = 'broadcast_id_example' # Συμβολοσειρά | 
opts = {
  edit_key: 'edit_key_example', # Συμβολοσειρά | 
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->delete_comment_vote: #{e}"
end
[inline-code-end]