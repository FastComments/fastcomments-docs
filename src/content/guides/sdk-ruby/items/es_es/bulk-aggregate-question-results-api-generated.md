## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| forceRecalculate | boolean | query | No |  |

## Respuesta

Devuelve: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_aggregate_question_results_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'bulk_aggregate_question_results Ejemplo'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar la autorización
FastCommentsClient.configure do |config|
  # Configurar la autorización con la clave de la API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente la siguiente línea para establecer un prefijo para la clave de la API, por ejemplo 'Bearer' (por defecto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
bulk_aggregate_question_results_request = FastCommentsClient::BulkAggregateQuestionResultsRequest.new({aggregations: [FastCommentsClient::BulkAggregateQuestionItem.new({agg_id: 'agg_id_example'})]}) # BulkAggregateQuestionResultsRequest | 
opts = {
  force_recalculate: true # Boolean | 
}

begin
  
  result = api_instance.bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->bulk_aggregate_question_results: #{e}"
end
[inline-code-end]