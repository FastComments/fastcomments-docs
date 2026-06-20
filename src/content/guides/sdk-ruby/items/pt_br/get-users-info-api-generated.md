Informações em massa de usuários para um tenant. Com os userIds fornecidos, retorna informações de exibição de User / SSOUser.
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.
Sem contexto de página: a privacidade é aplicada de forma uniforme (perfis privados são mascarados).

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| ids | string | query | Sim | userIds separados por vírgula. |

## Resposta

Retorna: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | userIds separados por vírgula.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]