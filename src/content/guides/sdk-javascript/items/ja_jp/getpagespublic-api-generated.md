テナントのページ一覧を取得します。FChat デスクトップクライアントがルームリストを表示するために使用します。
各ページの解決済みカスタム設定で `enableFChat` が true である必要があります。
SSO が必要なページは、リクエストユーザーのグループアクセス権に基づいてフィルタリングされます。

## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| cursor | string | いいえ |  |
| limit | number | いいえ |  |
| q | string | いいえ |  |
| sortBy | PagesSortBy | いいえ |  |
| hasComments | boolean | いいえ |  |

## レスポンス

戻り値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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