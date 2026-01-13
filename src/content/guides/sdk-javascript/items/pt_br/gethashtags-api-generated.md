## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| page | number | Não |  |

## Resposta

Retorna: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-7a9f";
  const tagsPage1: GetHashTags200Response = await getHashTags(tenantId);
  const tagsPage2: GetHashTags200Response = await getHashTags(tenantId, 2);
  console.log(tagsPage1, tagsPage2);
})();
[inline-code-end]

---