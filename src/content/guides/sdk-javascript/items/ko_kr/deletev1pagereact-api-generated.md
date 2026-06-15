## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`DeleteV1PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV1PageReact200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteV1PageReact 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = process.env.TENANT_ID ?? 'd3b07384-9f6a-4c2b-8c3e-0a1b2c3d4e5f';
const urlId: string = 'https://acme.com/articles/2026/06/fastcomments-integration';
const result: DeleteV1PageReact200Response = await deleteV1PageReact(tenantId, urlId);
[inline-code-end]

---