## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| includeEmail | boolean | query | Nein |  |
| includeIP | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Response

Rückgabe: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_comment_response.rb)

## Example

[inline-code-attrs-start title = 'Beispiel für get_moderation_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Zeichenkette | 
comment_id = 'comment_id_example' # Zeichenkette | 
opts = {
  include_email: true, # Boolesch | 
  include_ip: true, # Boolesch | 
  sso: 'sso_example' # Zeichenkette | 
}

begin
  
  result = api_instance.get_moderation_comment(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_moderation_comment: #{e}"
end
[inline-code-end]