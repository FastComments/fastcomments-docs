## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| includeEmail | boolean | query | Не |  |
| includeIP | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_comment_response.rb)

## Пример

[inline-code-attrs-start title = 'get_moderation_comment Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
opts = {
  include_email: true, # Boolean | 
  include_ip: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_moderation_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_moderation_comment: #{e}"
end
[inline-code-end]