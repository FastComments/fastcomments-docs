---
Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.
Use isto após esgotar /users/online para renderizar uma seção "Membros".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
a partir de afterName para frente via $gt, sem custo de $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | No | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | No | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nomes não excluam entradas. |

## Response

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Example

[inline-code-attrs-start title = 'get_offline_users Exemplo'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identificador da URL da página (limpo no servidor).
opts = {
  after_name: 'after_name_example', # String | Cursor: passe nextAfterName da resposta anterior.
  after_user_id: 'after_user_id_example' # String | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nomes não excluam entradas.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---