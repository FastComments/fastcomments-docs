## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Sim |  |
| userId | string | query | Não |  |
| anonUserId | string | query | Não |  |

## Resposta

Retorna: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_votes_for_user200_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_votes_for_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorização
FastCommentsClient.configure do |config|
  # Configurar autorização por chave de API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente a linha abaixo para definir um prefixo para a chave de API, por exemplo 'Bearer' (padrão: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  anon_user_id: 'anon_user_id_example' # String | 
}

begin
  
  result = api_instance.get_votes_for_user(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_votes_for_user: #{e}"
end
[inline-code-end]

---