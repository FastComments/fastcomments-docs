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

Επιστρέφει: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction παραλείφθηκε
  undefined, // repliesToUserId παραλείφθηκε
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---