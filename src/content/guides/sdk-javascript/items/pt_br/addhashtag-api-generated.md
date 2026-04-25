## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Não |  |
| createHashTagBody | CreateHashTagBody | Não |  |

## Resposta

Retorna: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo addHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string | undefined = undefined;
const createHashTagBody: CreateHashTagBody = {
  name: 'release-2026',
  description: 'Feedback and bug reports for the April 2026 product release',
  synonyms: ['v2-release', 'launch-2026'],
  color: '#1d72b8',
  isActive: true,
  createdBy: 'product.manager@acme-corp.com'
};
const result: AddHashTag200Response = await addHashTag(tenantId, createHashTagBody);
[inline-code-end]

---