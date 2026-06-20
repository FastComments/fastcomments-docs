## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| urlId | string | query | Não |  |
| fromCommentId | string | query | Não |  |
| viewed | boolean | query | Não |  |
| type | string | query | Não |  |

## Resposta

Retorna: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_notification_count_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_notification_count'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorização
FastCommentsClient.configure do |config|
  # Configurar autorização por chave de API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente a linha abaixo para definir um prefixo para a chave da API, ex.: 'Bearer' (padrão: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  url_id: 'url_id_example', # String | 
  from_comment_id: 'from_comment_id_example', # String | 
  viewed: true, # Boolean | 
  type: 'type_example' # String | 
}

begin
  
  result = api_instance.get_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_notification_count: #{e}"
end
[inline-code-end]