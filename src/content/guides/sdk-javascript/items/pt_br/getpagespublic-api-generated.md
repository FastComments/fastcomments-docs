Listar páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.  
Requer `enableFChat` ser true na configuração personalizada resolvida para cada página.  
Páginas que requerem SSO são filtradas com base no acesso ao grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| cursor | string | Não |  |
| limit | number | Não |  |
| q | string | Não |  |
| sortBy | PagesSortBy | Não |  |
| hasComments | boolean | Não |  |

## Resposta

Retorna: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]