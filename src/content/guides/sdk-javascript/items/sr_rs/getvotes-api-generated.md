## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Одговор

Враћа: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Пример

[inline-code-attrs-start title = 'Primer getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // Пример приступања опционој својини из одговора
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]