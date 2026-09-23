---
ページ上の過去のコメント投稿者で、現在オンラインではないもの。displayName でソートされます。  
`/users/online` をすべて取得した後に、"Members" セクションを表示するために使用します。  
commenterName に対するカーソルページネーション: サーバーは部分的な `{tenantId, urlId, commenterName}` インデックスを `afterName` から `$gt` で前方に進め、`$skip` コストはかかりません。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]

---