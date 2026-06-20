## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | ścieżka | Tak |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_ban_status_response.rb)

## Przykład

[inline-code-attrs-start title = 'get_comment_ban_status Przykład'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Ciąg znaków | 
opts = {
  sso: 'sso_example' # Ciąg znaków | 
}

begin
  
  result = api_instance.get_comment_ban_status(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_comment_ban_status: #{e}"
end
[inline-code-end]

---