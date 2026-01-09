[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Essa API é usada para obter comentários para exibição a um usuário. Por exemplo, ela filtra automaticamente comentários não aprovados ou spam.

### Pagination

A paginação pode ser feita de duas maneiras, dependendo dos requisitos de desempenho e do caso de uso:

1. Mais rápido: **Paginação Precalculada**:
   1. É assim que o FastComments funciona quando você usa nossos widgets e clientes pré-construídos.
   2. Ao clicar em "next" simplesmente aumenta a contagem de páginas.
   3. Você pode pensar nisso como sendo recuperado por um armazenamento chave-valor.
   4. Dessa forma, defina simplesmente um parâmetro `page` começando em `0` e uma direção de ordenação como `direction`.
   5. Os tamanhos de página podem ser personalizados via regras de personalização.
2. Mais flexível: **Paginação Flexível**:
   1. Dessa forma você pode definir parâmetros personalizados `limit` e `skip`. Não passe `page`.
   2. A ordenação `direction` também é suportada.
   3. `limit` é o total a ser retornado após a aplicação de `skip`.
      - Exemplo: defina `skip = 200, limit = 100` quando `page size = 100` e `page = 2`.
   4. Comentários filhos ainda contam na paginação. Você pode contornar isso usando a opção `asTree`.
      - Você pode paginar filhos via `limitChildren` e `skipChildren`.
      - Você pode limitar a profundidade das threads retornadas via `maxTreeDepth`.

### Threads

1. Ao usar `Precalculated Pagination`, os comentários são agrupados por *page* e comentários em threads afetam a página como um todo.
   1. Dessa forma, as threads podem ser determinadas no cliente com base em `parentId`.
   2. Por exemplo, em uma página com um comentário de nível superior e 29 respostas, ao definir `page=0` na API — você obterá apenas o comentário de nível superior e os 29 filhos.
   3. [Exemplo de imagem aqui ilustrando múltiplas páginas.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Ao usar `Flexible Pagination`, você pode definir um parâmetro `parentId`.
   1. Defina isso como null para obter apenas comentários de nível superior.
   2. Então, para visualizar threads, chame a API novamente e passe `parentId`.
   3. Uma solução comum é fazer uma chamada à API para os comentários de nível superior e então fazer chamadas paralelas à API para obter comentários dos filhos de cada comentário.
3. __NOVO a partir de fev de 2023!__ Busque como árvore usando `&asTree=true`.
   1. Você pode pensar nisso como `Flexible Pagination as a Tree`.
   2. Apenas os comentários de nível superior contam na paginação.
   3. Defina `parentId=null` para iniciar a árvore na raiz (você deve definir `parentId`).
   4. Defina `skip` e `limit` para paginação.
   5. Defina `asTree` como `true`.
   6. O custo de créditos aumenta em `2x`, pois nosso backend precisa fazer muito mais trabalho nesse cenário.
   7. Defina `maxTreeDepth`, `limitChildren` e `skipChildren` conforme desejado.

### Trees Explained

Ao usar `asTree`, pode ser difícil raciocinar sobre a paginação. Aqui está um gráfico útil:

<div class="screenshot white-bg">
    <div class="title">Diagrama de Paginação em Árvore</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagrama de Paginação em Árvore" />
</div>

### Fetching Comments in The Context of a User

A API `/comments` pode ser usada em dois contextos, para diferentes casos de uso:

- Para retornar comentários ordenados e marcados com informações para construir seu próprio cliente.
  - Nesse caso, defina um parâmetro de query `contextUserId`.
- Para buscar comentários a partir do seu backend para integrações customizadas.
  - A plataforma assumirá este caso por padrão sem `contextUserId`. 

[inline-code-attrs-start title = 'Paginação Precalculada de Comentários'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Paginação Flexível de Comentários'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Paginação Flexível de Comentários no Contexto do Usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Paginação Flexível de Comentários no Contexto do Usuário (Apenas Comentários de Primeiro Nível)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

É possível obter os comentários retornados como uma árvore, com a paginação contando apenas os comentários de nível superior.

[inline-code-attrs-start title = 'Comentários em Árvore no Contexto do Usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Quer obter apenas os comentários de nível superior e os filhos imediatos? Aqui está uma maneira:

[inline-code-attrs-start title = 'Comentários em Árvore com Profundidade Máx.'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

No entanto, na sua UI pode ser necessário saber se deve exibir um botão "mostrar respostas" em cada comentário. Ao buscar comentários via árvore, existe uma propriedade `hasChildren` adicionada aos comentários quando aplicável.

### Get Comments as a Tree, Searching by Hash Tag

É possível buscar por hashtag usando a API, em todo o seu tenant (não limitado a uma página, ou `urlId`).

Neste exemplo, omitimos `urlId`, e buscamos por múltiplas hashtags. A API retornará apenas comentários que contenham todas as hashtags requisitadas.

[inline-code-attrs-start title = 'Comentários em Árvore no Contexto do Usuário, por Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Estrutura de Requisição de Comentários'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** O urlId (URL da página, ou id do artigo) ao qual os comentários estão associados. **/
    urlId?: string
    /** Limita os comentários retornados por este usuário. **/
    userId?: string
    /** Use isto para buscar por hashtag. Para filtrar pela interseção de múltiplas hashtags, faça &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** A direção de ordenação. O padrão é MR (Mais Relevante). Outras opções são OF (Mais Antigos Primeiro) e NF (Mais Recentes Primeiro). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Paginação Precalculada: A página a ser buscada, começando em 0. Passe -1 para todos os comentários (até 250). **/
    page?: number
    /** Paginação Flexível: Quantos comentários devemos retornar? **/
    limit?: number
    /** Paginação Flexível: Quantos comentários filhos devemos retornar para cada pai? **/
    limitChildren?: number
    /** Paginação Flexível: Quantos comentários devemos pular? **/
    skip?: number
    /** Paginação Flexível: Quantos comentários filhos devemos pular para cada pai? **/
    skipChildren?: number
    /** Para determinar comentários bloqueados e sinalizados. **/
    contextUserId?: string
    /** Para determinar comentários bloqueados e sinalizados. **/
    anonUserId?: string
    /** Para buscar comentários filhos. **/
    parentId?: string
    /** Para buscar como uma árvore. **/
    asTree?: boolean
    /** Até que profundidade na árvore devemos retornar dados? 0 não retorna filhos. 1 retorna filhos imediatos, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Estrutura de Resposta de Comentários'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Os comentários! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Provavelmente você vai querer usar a API `Comment` com o parâmetro `urlId`. Você pode chamar a API `Pages` primeiro, para ver como os valores de `urlId` disponíveis para você se parecem. 

#### Anonymous Actions

Para comentários anônimos, provavelmente você vai querer passar `anonUserId` ao buscar comentários, e ao realizar sinalizações e bloqueios.

(!) Isto é exigido por muitas lojas de aplicativos, pois os usuários devem ser capazes de sinalizar conteúdo criado por usuários que eles podem ver, mesmo que não estejam logados. Não fazer isso pode causar a remoção do seu app dessa loja.

#### Comments Not Being Returned

Verifique se seus comentários estão aprovados e não são spam.