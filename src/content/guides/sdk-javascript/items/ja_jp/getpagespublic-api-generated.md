List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| cursor | string | いいえ |  |
| limit | number | いいえ |  |
| q | string | いいえ |  |
| sortBy | PagesSortBy | いいえ |  |
| hasComments | boolean | いいえ |  |

## レスポンス

返却: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## 例

[inline-code-attrs-start title = 'getPagesPublic 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]