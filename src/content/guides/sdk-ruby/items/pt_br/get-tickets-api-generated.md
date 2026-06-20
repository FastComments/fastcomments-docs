## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| state | number | query | Não |  |
| skip | number | query | Não |  |
| limit | number | query | Não |  |

## Resposta

Retorna: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tickets_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_tickets'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configurar autorização
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente a linha seguinte para definir um prefixo para a chave de API, por exemplo 'Bearer' (padrão: nil)
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

---