## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| id | string | Ja |  |

## Response

Retourneert: [`DeleteV2PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReactResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteV2PageReact Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "page_98765";
const reactionId: string = "react_abcde";

const deleteResult: DeleteV2PageReactResponse = await deleteV2PageReact(tenantId, urlId, reactionId);
[inline-code-end]