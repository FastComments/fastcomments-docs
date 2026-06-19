## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| userId | string | Hayır |  |
| direction | SortDirections | Hayır |  |
| repliesToUserId | string | Hayır |  |
| page | number | Hayır |  |
| includei10n | boolean | Hayır |  |
| locale | string | Hayır |  |
| isCrawler | boolean | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsForUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const userId: string = 'user_7421';
  const direction: SortDirections = SortDirections.Newest;
  const page: number = 2;
  const includei10n: boolean = true;
  const locale: string = 'en-GB';
  const isCrawler: boolean = false;
  const response: GetCommentsForUserResponse = await getCommentsForUser(userId, direction, undefined, page, includei10n, locale, isCrawler);
  console.log(response);
})();
[inline-code-end]

---