## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | yol | Evet |  |
| includeEmail | boolean | sorgu | Hayır |  |
| includeIP | boolean | sorgu | Hayır |  |
| sso | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_comment_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_moderation_comment Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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

---