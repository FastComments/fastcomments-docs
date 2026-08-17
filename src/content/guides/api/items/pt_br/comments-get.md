[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Esta API é usada para obter comentários para exibição ao usuário. Por exemplo, ela filtra automaticamente comentários não aprovados ou de spam.

### Paginação

Paginação pode ser feita de duas maneiras, dependendo dos requisitos de desempenho e caso de uso:

1. **Mais rápido: **Paginação Precalculada**:
   1. É assim que o FastComments funciona quando você usa nossos widgets e clientes pré-construídos.
   2. Clicar em "next" simplesmente aumenta a contagem de páginas.
   3. Você pode pensar nisso como sendo recuperado por um armazenamento chave-valor.
   4. Dessa forma, basta definir um parâmetro `page` começando em `0` e uma direção de ordenação como `direction`.
   5. Os tamanhos de página podem ser personalizados via regras de customização.
2. **Mais flexível: **Paginação Flexível**:
   1. Dessa forma, você pode definir parâmetros personalizados `limit` e `skip`. Não passe `page`.
   2. `direction` de ordenação também é suportado.
   3. `limit` é o número total a ser retornado após a aplicação de `skip`.
      - Exemplo: defina `skip = 200, limit = 100` quando `page size = 100` e `page = 2`.
   4. Comentários filhos ainda contam na paginação. Você pode contornar isso usando a opção `asTree`.
      - Você pode paginar filhos via `limitChildren` e `skipChildren`.
      - Você pode limitar a profundidade dos tópicos retornados via `maxTreeDepth`.

### Tópicos

1. Ao usar `Paginação Precalculada`, os comentários são agrupados por *página* e os comentários nos tópicos afetam a página geral.
   1. Dessa forma, os tópicos podem ser determinados no cliente com base em `parentId`.
   2. Por exemplo, com uma página contendo um comentário de nível superior e 29 respostas, e definindo `page=0` na API – você receberá apenas o comentário de nível superior e os 29 filhos.
2. Ao usar `Paginação Flexível`, você pode definir um parâmetro `parentId`.
   1. Defina isso como null para obter apenas comentários de nível superior.
   2. Então, para visualizar os tópicos, chame a API novamente e passe `parentId`.
   3. Uma solução comum é fazer uma chamada de API para os comentários de nível superior e então fazer chamadas paralelas de API para obter os comentários dos filhos de cada comentário.
3. __NOVO A partir de fev de 2023!__ Busque como árvore usando `&asTree=true`.
   1. Você pode pensar nisso como `Paginação Flexível como Árvore`.
   2. Apenas os comentários de nível superior contam na paginação.
   3. Defina `parentId=null` para iniciar a árvore na raiz (você deve definir `parentId`).
   4. Defina `skip` e `limit` para paginação.
   5. Defina `asTree` como `true`.
   6. O custo de créditos aumenta em `2x`, pois nosso backend precisa fazer muito mais trabalho neste cenário.
   7. Defina `maxTreeDepth`, `limitChildren` e `skipChildren` conforme desejado.

### Árvores Explicadas

Ao usar `asTree`, pode ser difícil raciocinar sobre paginação. Aqui está um gráfico útil:

<div class="screenshot white-bg">
    <div class="title">Diagrama de Paginação de Árvore</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagrama de Paginação de Árvore" />
</div>

### Obtendo Comentários no Contexto de um Usuário

A API `/comments` pode ser usada em dois contextos, para diferentes casos de uso:

- Para retornar comentários ordenados e marcados com informações para construir seu próprio cliente.
  - Neste caso, defina um parâmetro de consulta `contextUserId`.
- Para buscar comentários do seu backend para integrações personalizadas.
  - A plataforma usará isso por padrão sem `contextUserId`. 

[inline-code-attrs-start title = 'Comentários Paginação Precalculada'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Comentários Paginação Flexível'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Comentários Paginação Flexível no Contexto do Usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Comentários Paginação Flexível no Contexto do Usuário Apenas Comentários de Nível Superior'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Comentários como Árvore

É possível obter os comentários retornados como uma árvore, com paginação contando apenas os comentários de nível superior.

[inline-code-attrs-start title = 'Comentários Como Árvore no Contexto do Usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Quer obter apenas os comentários de nível superior e os filhos imediatos? Aqui está uma maneira:

[inline-code-attrs-start title = 'Comentários Como Árvore com Profundidade Máxima'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

No entanto, na sua UI você pode precisar saber se deve mostrar um botão "mostrar respostas" em cada comentário. Ao buscar comentários via uma árvore há uma propriedade `hasChildren` marcada nos comentários quando aplicável.

### Comentários como Árvore, Buscando por Hashtag

É possível buscar por hashtag usando a API, em todo o seu tenant (não limitado a uma página, ou `urlId`).

Neste exemplo, omitimos `urlId` e buscamos por múltiplas hashtags. A API retornará apenas comentários que possuam todas as hashtags solicitadas.

[inline-code-attrs-start title = 'Comentários Como Árvore no Contexto do Usuário, Por Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Estrutura da Requisição de Comentários

[inline-code-attrs-start title = 'Estrutura da Requisição de Comentários'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** O urlId (URL da página ou ID do artigo) ao qual os comentários estão associados. **/
    urlId?: string
    /** Limita os comentários retornados por este usuário. **/
    userId?: string
    /** Use isso para buscar por hashtag. Para aprofundar na interseção de múltiplas hashtags, use &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** A direção de ordenação. O padrão é MR (Mais Relevante). Outras opções são OF (Mais Antigos Primeiro) e NF (Mais Recentes Primeiro). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Paginação Precalculada: A página a buscar, começando em 0. Passe -1 para todos os comentários (até 250). **/
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
    /** Para buscar como árvore. **/
    asTree?: boolean
    /** Até que profundidade da árvore devemos retornar dados? 0 não retorna filhos. 1 retorna filhos imediatos, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Estrutura da Resposta de Comentários

[inline-code-attrs-start title = 'Estrutura da Resposta de Comentários'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Dicas úteis

#### ID da URL

Provavelmente você quer usar a API `Comment` com o parâmetro `urlId`. Você pode chamar a API `Pages` primeiro, para ver como são os valores de `urlId` disponíveis para você. 

#### Ações anônimas

Para comentários anônimos, provavelmente você quer passar `anonUserId` ao buscar comentários, e ao realizar sinalizações e bloqueios.

(!) Isso é obrigatório em muitas lojas de aplicativos, pois os usuários devem poder sinalizar conteúdo criado por usuários que eles podem ver, mesmo que não estejam logados. Não fazer isso pode fazer com que seu aplicativo seja removido da referida loja.

#### Comentários não sendo retornados

Verifique se seus comentários estão aprovados e não são spam.

---