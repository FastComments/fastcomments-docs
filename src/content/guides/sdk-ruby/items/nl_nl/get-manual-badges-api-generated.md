## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_manual_badges_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_manual_badges Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_manual_badges(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Fout bij het aanroepen van ModerationApi->get_manual_badges: #{e}"
end
[inline-code-end]