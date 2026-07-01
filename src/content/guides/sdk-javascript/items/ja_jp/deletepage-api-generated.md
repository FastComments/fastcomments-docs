## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返り値: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePageAPIResponse.ts)

## 例

[inline-code-attrs-start title = 'deletePage 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
    const tenantId: string = "acme-corp-001";
    const pageId: string = "page-987654321";
    const result: DeletePageAPIResponse = await deletePage(tenantId, pageId);
    console.log(result);
}
example();
[inline-code-end]