---
## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Ja |  |
| id | string | path | Ja |  |

## Svar

Returnerer: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_ticket_state200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'change_ticket_state Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsætning af autorisation
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgleautorisering: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentaren fra følgende linje for at angive et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
id = 'id_example' # String | 
change_ticket_state_body = FastCommentsClient::ChangeTicketStateBody.new({state: 37}) # ChangeTicketStateBody | 

begin
  
  result = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->change_ticket_state: #{e}"
end
[inline-code-end]

---