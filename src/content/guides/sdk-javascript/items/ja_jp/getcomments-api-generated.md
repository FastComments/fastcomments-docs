## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| page | number | いいえ |  |
| limit | number | いいえ |  |
| skip | number | いいえ |  |
| asTree | boolean | いいえ |  |
| skipChildren | number | いいえ |  |
| limitChildren | number | いいえ |  |
| maxTreeDepth | number | いいえ |  |
| urlId | string | いいえ |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |
| contextUserId | string | いいえ |  |
| hashTag | string | いいえ |  |
| parentId | string | いいえ |  |
| direction | SortDirections | いいえ |  |

## レスポンス

戻り値: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## 例

[inline-code-attrs-start title = 'getComments の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // ページ
  20, // 取得件数
  0, // スキップ数
  true, // ツリーとして取得するか
  1, // スキップする子の数
  3, // 子の取得上限
  4, // ツリーの最大深度
  'articles/2026/new-product-launch', // urlId（URL の識別子）
  'user_7890', // userId（ユーザーID）
  'anon_4f3b2', // anonUserId（匿名ユーザーID）
  undefined, // contextUserId
  '#launch', // ハッシュタグ
  undefined // parentId
);
[inline-code-end]