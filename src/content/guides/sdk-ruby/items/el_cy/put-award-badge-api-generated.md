## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| commentId | string | query | Όχι |  |
| broadcastId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/award_user_badge_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα put_award_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
badge_id = 'badge_id_example' # Συμβολοσειρά | 
opts = {
  user_id: 'user_id_example', # Συμβολοσειρά | 
  comment_id: 'comment_id_example', # Συμβολοσειρά | 
  broadcast_id: 'broadcast_id_example', # Συμβολοσειρά | 
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.put_award_badge(badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_award_badge: #{e}"
end
[inline-code-end]

---