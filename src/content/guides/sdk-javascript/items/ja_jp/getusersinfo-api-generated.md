テナントの一括ユーザー情報。userIds を指定すると、User / SSOUser から表示用情報を返します。
コメントウィジェットで使用され、プレゼンスイベントで出現したユーザーの情報を補完します。
ページコンテキストがないため、プライバシーは一律に適用されます（非公開プロフィールはマスクされます）。

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## レスポンス

戻り値: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // 任意。undefined の場合はデフォルトでカンマが使用されます
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---