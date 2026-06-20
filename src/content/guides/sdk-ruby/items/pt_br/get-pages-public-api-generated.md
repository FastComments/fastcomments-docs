Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.
Exige que `enableFChat` seja true na configuração personalizada resolvida para cada página.
Páginas que exigem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, padrão 50 |
| q | string | query | Não | Filtro opcional por prefixo de título, insensível a maiúsculas/minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética). |
| hasComments | boolean | query | Não | Se true, apenas retorna páginas com pelo menos um comentário. |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`.
  limit: 56, # Integer | 1..200, padrão 50
  q: 'q_example', # String | Filtro opcional por prefixo de título, insensível a maiúsculas/minúsculas.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética).
  has_comments: true # Boolean | Se true, apenas retorna páginas com pelo menos um comentário.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]