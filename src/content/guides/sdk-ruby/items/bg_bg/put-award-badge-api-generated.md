## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| badgeId | string | query | Да |  |
| userId | string | query | Не |  |
| commentId | string | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/award_user_badge_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример put_award_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
badge_id = 'badge_id_example' # Низ | 
opts = {
  user_id: 'user_id_example', # Низ | 
  comment_id: 'comment_id_example', # Низ | 
  broadcast_id: 'broadcast_id_example', # Низ | 
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.put_award_badge(badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_award_badge: #{e}"
end
[inline-code-end]