## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de update_feed_post'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorização
FastCommentsClient.configure do |config|
  # Configurar autorização por chave de API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Uncomment the following line to set a prefix for the API key, e.g. 'Bearer' (defaults to nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
feed_post = FastCommentsClient::FeedPost.new({_id: '_id_example', tenant_id: 'tenant_id_example', created_at: Time.now}) # FeedPost | 

begin
  
  result = api_instance.update_feed_post(tenant_id, id, feed_post)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_feed_post: #{e}"
end
[inline-code-end]