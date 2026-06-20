## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| fromName | string | query | Ja |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Eksempel

[inline-code-attrs-start title = 'send_invite Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsæt godkendelse
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgle-godkendelse: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentartegnet på følgende linje for at sætte et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
from_name = 'from_name_example' # String | 

begin
  
  result = api_instance.send_invite(tenant_id, id, from_name)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->send_invite: #{e}"
end
[inline-code-end]

---