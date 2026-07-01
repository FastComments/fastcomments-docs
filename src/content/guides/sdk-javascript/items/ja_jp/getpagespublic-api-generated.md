List pages for a tenant. Used by the FChat desktop client to populate its room list.  
テナントのページを一覧取得します。FChat デスクトップクライアントが部屋リストを表示するために使用します。

Requires `enableFChat` to be true on the resolved custom config for each page.  
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。

Pages that require SSO are filtered against the requesting user's group access.  
SSO が必要なページは、リクエストするユーザーのグループアクセスに基づいてフィルタリングされます。

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| cursor | string | いいえ |  |
| limit | number | いいえ |  |
| q | string | いいえ |  |
| sortBy | PagesSortBy | いいえ |  |
| hasComments | boolean | いいえ |  |

## Response

返却: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Example

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]