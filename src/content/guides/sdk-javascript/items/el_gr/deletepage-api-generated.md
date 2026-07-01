## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePageAPIResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'deletePage Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
    const tenantId: string = "acme-corp-001";
    const pageId: string = "page-987654321";
    const result: DeletePageAPIResponse = await deletePage(tenantId, pageId);
    console.log(result);
}
example();
[inline-code-end]