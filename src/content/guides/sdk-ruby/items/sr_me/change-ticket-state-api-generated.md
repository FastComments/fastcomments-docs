---
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_ticket_state_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer change_ticket_state'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfiguriši autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sljedeću liniju da postavite prefiks za API ključ, npr. 'Bearer' (zadano je nil)
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