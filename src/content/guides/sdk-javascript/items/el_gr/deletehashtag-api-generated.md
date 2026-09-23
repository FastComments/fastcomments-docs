## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| tag | string | Ναι |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteHashTag Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const tag: string = "high-priority";

const requestBody: DeleteHashTagRequestBody = {
  // συμπληρώστε τα πεδία όπως χρειάζεται
};

const resultWithBody: APIEmptyResponse = await deleteHashTag(tenantId, tag, requestBody);
const resultWithoutBody: APIEmptyResponse = await deleteHashTag(tenantId, tag);
[inline-code-end]