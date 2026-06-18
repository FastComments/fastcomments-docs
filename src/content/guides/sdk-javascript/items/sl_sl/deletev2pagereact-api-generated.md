## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrne: [`DeleteV2PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReact200Response.ts)

## Primer

[inline-code-attrs-start title = 'deleteV2PageReact Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_79021";
const urlId: string = "blog/my-first-post";
const id: string = "reaction_9f8b7c";
let includeHistory: boolean | undefined = undefined; // neobvezna zastavica, uporabljena pri nekaterih klicih

const result: DeleteV2PageReact200Response = await deleteV2PageReact(tenantId, urlId, id);
console.log(result);
[inline-code-end]

---