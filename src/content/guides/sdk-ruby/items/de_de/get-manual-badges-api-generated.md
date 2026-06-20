---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_manual_badges_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_manual_badges Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_manual_badges(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_manual_badges: #{e}"
end
[inline-code-end]

---