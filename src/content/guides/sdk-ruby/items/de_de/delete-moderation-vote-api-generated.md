## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | Pfad | Ja |  |
| voteId | string | Pfad | Ja |  |
| sso | string | Abfrage | Nein |  |

## Antwort

Gibt zurück: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_delete_response.rb)

## Beispiel

[inline-code-attrs-start title = 'delete_moderation_vote Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
vote_id = 'vote_id_example' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.delete_moderation_vote(comment_id, vote_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->delete_moderation_vote: #{e}"
end
[inline-code-end]

---