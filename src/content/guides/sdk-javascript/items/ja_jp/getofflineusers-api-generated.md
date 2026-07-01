過去にページ上でコメントしたが、現在オンラインではないユーザー。displayName でソートされます。  
/​users/online をすべて使用し終えた後、"Members" セクションを表示するために使用します。  
commenterName に対するカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName} インデックスを afterName から $gt で前方向に走査し、$skip コストはかかりません。

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

返却: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]