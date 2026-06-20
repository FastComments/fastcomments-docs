## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_comment_text_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer post_set_comment_text'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | ID komentara
set_comment_text_params = FastCommentsClient::SetCommentTextParams.new({comment: 'comment_example'}) # SetCommentTextParams | Parametri za postavljanje teksta komentara
opts = {
  sso: 'sso_example' # String | SSO token
}

begin
  
  result = api_instance.post_set_comment_text(comment_id, set_comment_text_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_text: #{e}"
end
[inline-code-end]