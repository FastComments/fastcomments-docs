## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| locale | string | Não |  |
| rating | string | Não |  |
| page | number | Não |  |

## Resposta

Retorna: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-01";
const trendingBasic: GifSearchResponse = await getGifsTrending(tenantId);

const locale: string = "en-GB";
const rating: string = "pg";
const page: number = 1;
const trendingWithOptions: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---