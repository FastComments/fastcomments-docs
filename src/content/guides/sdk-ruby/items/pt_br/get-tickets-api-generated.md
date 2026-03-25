---
## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Resposta

Retorna: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tickets200_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_tickets'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorização
FastCommentsClient.configure do |config|
  # Configurar autorização por chave de API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente a linha a seguir para definir um prefixo para a chave de API, por exemplo 'Bearer' (padrão: nil)
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