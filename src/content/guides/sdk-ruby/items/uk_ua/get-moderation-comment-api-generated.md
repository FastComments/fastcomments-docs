## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | шлях | Так |  |
| includeEmail | boolean | параметр запиту | Ні |  |
| includeIP | boolean | параметр запиту | Ні |  |
| sso | string | параметр запиту | Ні |  |

## Відповідь

Повертає: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_comment_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_moderation_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Рядок | 
opts = {
  include_email: true, # Булеве | 
  include_ip: true, # Булеве | 
  sso: 'sso_example' # Рядок | 
}

begin
  
  result = api_instance.get_moderation_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_moderation_comment: #{e}"
end
[inline-code-end]

---