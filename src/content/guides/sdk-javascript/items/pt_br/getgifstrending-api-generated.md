## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| locale | string | Não |  |
| rating | string | Não |  |
| page | number | Não |  |

## Resposta

Retorna: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media';
const locale: string = 'en-US';
const rating: string = 'pg-13';
const page: number = 2;
const trending: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---