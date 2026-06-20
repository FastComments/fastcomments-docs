## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| broadcastId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_comment_pin_status_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio pin_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Stringa | 
comment_id = 'comment_id_example' # Stringa | 
broadcast_id = 'broadcast_id_example' # Stringa | 
opts = {
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.pin_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->pin_comment: #{e}"
end
[inline-code-end]