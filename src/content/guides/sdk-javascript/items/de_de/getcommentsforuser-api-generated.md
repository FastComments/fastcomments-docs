## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| userId | string | No |  |
| direction | SortDirections | No |  |
| repliesToUserId | string | No |  |
| page | number | No |  |
| includei10n | boolean | No |  |
| locale | string | No |  |
| isCrawler | boolean | No |  |

## Antwort

Rückgabe: [`GetCommentsForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
    const userId: string = "user-12345";
    const direction: SortDirections = "desc";
    const page: number = 1;
    const includei10n: boolean = true;
    const locale: string = "en-US";
    const isCrawler: boolean = false;

    const response: GetCommentsForUserResponse1 = await getCommentsForUser(
        userId,
        direction,
        undefined,
        page,
        includei10n,
        locale,
        isCrawler
    );

    console.log(response);
}
[inline-code-end]