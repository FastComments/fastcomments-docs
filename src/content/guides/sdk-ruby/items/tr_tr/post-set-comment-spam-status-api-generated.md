---
## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| spam | boolean | query | Hayır |  |
| permNotSpam | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Örnek

[inline-code-attrs-start title = 'post_set_comment_spam_status Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
opts = {
  spam: true, # Boolean | 
  perm_not_spam: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_set_comment_spam_status(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_spam_status: #{e}"
end
[inline-code-end]

---