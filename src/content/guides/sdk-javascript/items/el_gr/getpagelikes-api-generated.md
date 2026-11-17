Πλήθος "μου αρέσει" για μια σελίδα, προαιρετικά με τους χρήστες που έκαναν like.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| includeUserInfo | PageReactIncludeUserInfo | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// Βασική κλήση με τις απαιτούμενες παραμέτρους
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Page has ${likesResponse.pageLikes.length} likes`);

// Συμπερίληψη στοιχείων χρήστη για κάθε like
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} liked this page on ${like.date}`);
});
[inline-code-end]