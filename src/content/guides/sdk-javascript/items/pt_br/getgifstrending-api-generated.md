## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| locale | string | Não |  |
| rating | string | Não |  |
| page | number | Não |  |

## Resposta

Retorna: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const locale: string = 'en-US';
const rating: string = 'PG';
const page: number = 1;
const result: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---