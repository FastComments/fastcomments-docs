## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Risposta

Restituisce: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tickets200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio get_tickets'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurazione autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autenticazione tramite chiave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la riga seguente per impostare un prefisso per la chiave API, e.g. 'Bearer' (predefinito nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  state: 1.2, # Float | 
  skip: 1.2, # Float | 
  limit: 1.2 # Float | 
}

begin
  
  result = api_instance.get_tickets(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tickets: #{e}"
end
[inline-code-end]