## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| userId | string | Nee |  |
| direction | SortDirections | Nee |  |
| repliesToUserId | string | Nee |  |
| page | number | Nee |  |
| includei10n | boolean | Nee |  |
| locale | string | Nee |  |
| isCrawler | boolean | Nee |  |

## Response

Retourneert: [`GetCommentsForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse1.ts)

## Example

[inline-code-attrs-start title = 'getCommentsForUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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