---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-publishing-01';
const id: string = 'qres-7a3d9f45-2b6e-4c9a-8f1b-0d3f2c9a1e6b';
const result: APIEmptyResponse = await deleteQuestionResult(tenantId, id);
[inline-code-end]

---