## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Ja |  |

## Antwoord

Geeft terug: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_ticket200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'create_ticket Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer autorisatie via API-sleutel: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaartekens van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard is nil)
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