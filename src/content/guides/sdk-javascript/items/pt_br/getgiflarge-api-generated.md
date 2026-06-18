## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| largeInternalURLSanitized | string | Sim |  |

## Resposta

Retorna: [`GetGifLarge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifLarge200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c";
const largeInternalURLSanitized: string = "https://cdn.fastcomments.com/gifs/07d3f6_large.gif";
const preferWebP: boolean | undefined = true; // preferência opcional
const urlToUse: string = preferWebP ? largeInternalURLSanitized.replace(".gif", ".webp") : largeInternalURLSanitized;
const response: GetGifLarge200Response = await getGifLarge(tenantId, urlToUse);
[inline-code-end]

---