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

Döndürür: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsForUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction atlandı
  undefined, // repliesToUserId atlandı
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---