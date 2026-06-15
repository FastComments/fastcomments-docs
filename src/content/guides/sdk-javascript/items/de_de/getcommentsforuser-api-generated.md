---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| userId | string | Nein |  |
| direction | SortDirections | Nein |  |
| repliesToUserId | string | Nein |  |
| page | number | Nein |  |
| includei10n | boolean | Nein |  |
| locale | string | Nein |  |
| isCrawler | boolean | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction ausgelassen
  undefined, // repliesToUserId ausgelassen
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---