---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| userId | string | Non |  |
| direction | SortDirections | Non |  |
| repliesToUserId | string | Non |  |
| page | number | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| isCrawler | boolean | Non |  |

## Réponse

Renvoie : [`GetCommentsForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---