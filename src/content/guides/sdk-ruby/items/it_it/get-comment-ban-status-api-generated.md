## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_ban_status_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_comment_ban_status'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Stringa | 
opts = {
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.get_comment_ban_status(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_comment_ban_status: #{e}"
end
[inline-code-end]

---