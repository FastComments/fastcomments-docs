テナントのページを一覧表示します。FChat デスクトップクライアントがルームリストを生成するために使用します。
各ページの解決済みカスタム設定で `enableFChat` が true である必要があります。
SSO を必要とするページは、リクエストしているユーザーのグループアクセスに基づいてフィルタリングされます。

## パラメータ

| Name | Type | Required | Description |
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
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---