## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| userId | string | Όχι |  |
| direction | SortDirections | Όχι |  |
| repliesToUserId | string | Όχι |  |
| page | number | Όχι |  |
| includei10n | boolean | Όχι |  |
| locale | string | Όχι |  |
| isCrawler | boolean | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetCommentsForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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