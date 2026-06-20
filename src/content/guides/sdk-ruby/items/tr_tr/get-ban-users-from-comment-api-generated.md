## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Dönen değer: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_banned_users_from_comment_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_ban_users_from_comment Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_ban_users_from_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_ban_users_from_comment: #{e}"
end
[inline-code-end]

---