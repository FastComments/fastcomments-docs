## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| id | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`DeleteV2PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReact200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteV2PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_79021";
const urlId: string = "blog/my-first-post";
const id: string = "reaction_9f8b7c";
let includeHistory: boolean | undefined = undefined; // προαιρετική παράμετρος, χρησιμοποιείται σε ορισμένες κλήσεις

const result: DeleteV2PageReact200Response = await deleteV2PageReact(tenantId, urlId, id);
console.log(result);
[inline-code-end]

---