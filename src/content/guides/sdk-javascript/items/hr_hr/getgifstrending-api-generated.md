## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| locale | string | Ne |  |
| rating | string | Ne |  |
| page | number | Ne |  |

## Odgovor

Vraća: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getGifsTrending Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const locale: string = "en-US";
  const rating: string = "PG-13";
  const page: number = 1;

  const trendingAll: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
  console.log(trendingAll);

  // Koristeći samo obavezni parametar
  const trendingMinimal: GetGifsTrendingResponse = await getGifsTrending(tenantId);
  console.log(trendingMinimal);
}

runExample();
[inline-code-end]

---