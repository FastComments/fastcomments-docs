## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| userId | string | Όχι |  |
| direction | SortDirections | Όχι |  |
| repliesToUserId | string | Όχι |  |
| page | number | Όχι |  |
| includei10n | boolean | Όχι |  |
| locale | string | Όχι |  |
| isCrawler | boolean | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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