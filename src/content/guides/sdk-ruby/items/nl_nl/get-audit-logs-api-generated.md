## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| limit | number | query | Nee |  |
| skip | number | query | Nee |  |
| order | string | query | Nee |  |
| after | number | query | Nee |  |
| before | number | query | Nee |  |

## Response

Retourneert: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_audit_logs_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_audit_logs Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# authorisatie instellen
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal het commentaarteken van de volgende regel weg om een prefix voor de API-sleutel in te stellen, bv. 'Bearer' (standaard nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  limit: 1.2, # Float | 
  skip: 1.2, # Float | 
  order: FastCommentsClient::SORTDIR::ASC, # SORTDIR | 
  after: 1.2, # Float | 
  before: 1.2 # Float | 
}

begin
  
  result = api_instance.get_audit_logs(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_audit_logs: #{e}"
end
[inline-code-end]