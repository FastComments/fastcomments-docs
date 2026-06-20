## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| reviewed | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'post_set_comment_review_status Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Строка | 
opts = {
  reviewed: true, # Булево | 
  sso: 'sso_example' # Строка | 
}

begin
  
  result = api_instance.post_set_comment_review_status(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_review_status: #{e}"
end
[inline-code-end]