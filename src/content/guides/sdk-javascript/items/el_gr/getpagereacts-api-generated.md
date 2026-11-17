Προσαρμοσμένες μετρήσεις react για μια σελίδα, προαιρετικά με τους χρήστες που έκαναν react.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| includeUserInfo | PageReactIncludeUserInfo | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageReactsAPIResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'my-blog-post-42';
const includeUserInfo: PageReactIncludeUserInfo = true;

const response: GetPageReactsAPIResponse = await getPageReacts(
  tenantId,
  urlId,
  includeUserInfo
);

const reacts = response.pageReacts;
const reactCount: number = reacts.like.count;
const users: PageCustomReactUser[] = reacts.like.userDetails ?? [];
[inline-code-end]