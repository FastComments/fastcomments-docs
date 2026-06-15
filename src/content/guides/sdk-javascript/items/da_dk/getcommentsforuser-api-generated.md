## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| userId | string | Nej |  |
| direction | SortDirections | Nej |  |
| repliesToUserId | string | Nej |  |
| page | number | Nej |  |
| includei10n | boolean | Nej |  |
| locale | string | Nej |  |
| isCrawler | boolean | Nej |  |

## Svar

Returnerer: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction udeladt
  undefined, // repliesToUserId udeladt
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---