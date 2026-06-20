Aggrega documenti raggruppandoli (se viene fornito groupBy) e applicando più operazioni. Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Risposta

Restituisce: [`AggregateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/aggregate_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregate'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configurazione dell'autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autorizzazione con API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la riga seguente per impostare un prefisso per la API key, ad es. 'Bearer' (predefinito a nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
aggregation_request = FastCommentsClient::AggregationRequest.new({resource_name: 'resource_name_example', operations: [FastCommentsClient::AggregationOperation.new({field: 'field_example', op: FastCommentsClient::AggregationOpType::SUM})]}) # AggregationRequest | 
opts = {
  parent_tenant_id: 'parent_tenant_id_example', # String | 
  include_stats: true # Boolean | 
}

begin
  
  result = api_instance.aggregate(tenant_id, aggregation_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->aggregate: #{e}"
end
[inline-code-end]

---