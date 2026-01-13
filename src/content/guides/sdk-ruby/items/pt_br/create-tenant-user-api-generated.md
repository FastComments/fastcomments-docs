---
## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |

## Resposta

Retorna: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant_user200_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de create_tenant_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorização
FastCommentsClient.configure do |config|
  # Configurar autorização por chave de API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente a linha abaixo para definir um prefixo para a chave da API, por exemplo 'Bearer' (padrão: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_tenant_user_body = FastCommentsClient::CreateTenantUserBody.new({username: 'username_example', email: 'email_example'}) # CreateTenantUserBody | 

begin
  
  result = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant_user: #{e}"
end
[inline-code-end]

---