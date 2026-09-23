## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
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
const userId: string = '550e8400-e29b-41d4-a716-446655440000';
const direction: SortDirections = SortDirections.Descending;
const page: number = 3;
const includei10n: boolean = true;
const locale: string = 'fr-FR';
const isCrawler: boolean = false;

const commentsResponse: GetCommentsForUserResponse = await getCommentsForUser(
  userId,
  direction,
  undefined,
  page,
  includei10n,
  locale,
  isCrawler
);
[inline-code-end]

---