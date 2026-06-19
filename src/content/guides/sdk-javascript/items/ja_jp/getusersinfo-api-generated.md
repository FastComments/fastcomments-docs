テナントのユーザー情報を一括で取得します。userIds を指定すると User / SSOUser から表示情報を返します。
コメントウィジェットによって、presence event により新たに表示されたユーザー情報を充実させるために使用されます。
ページコンテキストがありません: プライバシーは一律に適用されます（非公開プロフィールはマスクされます）。

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
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo は tenantId と ids のみを要求します。オプションのパラメータはここでは該当しません。
[inline-code-end]

---