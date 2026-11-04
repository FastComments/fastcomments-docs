テナントのユーザー情報の一括取得。userIds を受け取り、User / SSOUser から表示情報を返します。
コメントウィジェットがプレゼンスイベント経由で新たに現れたユーザーを補完するために使用します。
ページコンテキストはありません: プライバシーは一律に適用されます(非公開プロフィールはマスクされます)。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| ids | string | はい |  |

## レスポンス

戻り値: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]