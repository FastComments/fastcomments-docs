## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vrne: [`DeleteV2PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReactResponse.ts)

## Primer

[inline-code-attrs-start title = 'deleteV2PageReact Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "page_98765";
const reactionId: string = "react_abcde";

const deleteResult: DeleteV2PageReactResponse = await deleteV2PageReact(tenantId, urlId, reactionId);
[inline-code-end]

---