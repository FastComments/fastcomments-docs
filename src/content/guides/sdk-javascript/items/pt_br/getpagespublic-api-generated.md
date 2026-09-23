Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas. Requer que `enableFChat` seja true na configuração personalizada resolvida para cada página. Páginas que requerem SSO são filtradas de acordo com o acesso ao grupo do usuário solicitante.

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

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]