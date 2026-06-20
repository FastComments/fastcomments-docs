---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postIds | array | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/user_reacts_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_user_reacts_public Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  post_ids: ['inner_example'], # Array<String> | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_reacts_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_reacts_public: #{e}"
end
[inline-code-end]

---