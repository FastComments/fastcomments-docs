## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| editKey | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comment_text200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_comment_text Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  edit_key: 'edit_key_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_comment_text(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_text: #{e}"
end
[inline-code-end]