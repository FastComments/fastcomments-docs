## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Не |  |
| commentId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_manual_badges_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за get_manual_badges_for_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  badges_user_id: 'badges_user_id_example', # Низ | 
  comment_id: 'comment_id_example', # Низ | 
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.get_manual_badges_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_manual_badges_for_user: #{e}"
end
[inline-code-end]

---