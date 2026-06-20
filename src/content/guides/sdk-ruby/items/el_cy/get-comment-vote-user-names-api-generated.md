## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| dir | integer | query | Yes |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_vote_user_names_success_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_comment_vote_user_names'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
comment_id = 'comment_id_example' # Συμβολοσειρά | 
dir = 56 # Ακέραιος | 
opts = {
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.get_comment_vote_user_names(tenant_id, comment_id, dir, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_vote_user_names: #{e}"
end
[inline-code-end]