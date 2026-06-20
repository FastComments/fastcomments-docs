## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Tak |  |
| userId | string | query | Nie |  |
| commentId | string | query | Nie |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/remove_user_badge_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład put_remove_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
badge_id = 'badge_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  comment_id: 'comment_id_example', # String | 
  broadcast_id: 'broadcast_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.put_remove_badge(badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_remove_badge: #{e}"
end
[inline-code-end]