## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | Sì |  |

## Risposta

Restituisce: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_ticket200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di create_ticket'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurazione dell'autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autorizzazione tramite API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la riga seguente per impostare un prefisso per la API key, es. 'Bearer' (predefinito: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
create_ticket_body = FastCommentsClient::CreateTicketBody.new({subject: 'subject_example'}) # CreateTicketBody | 

begin
  
  result = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_ticket: #{e}"
end
[inline-code-end]