## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| userId | string | Nein |  |
| direction | SortDirections | Nein |  |
| repliesToUserId | string | Nein |  |
| page | number | Nein |  |
| includei10n | boolean | Nein |  |
| locale | string | Nein |  |
| isCrawler | boolean | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getCommentsForUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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