---
テナントのページを一覧表示します。FChat デスクトップクライアントがそのルーム一覧を作成するために使用します。各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。SSO を必要とするページは、リクエスト元ユーザーのグループアクセスに対してフィルタリングされます。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| cursor | String | いいえ |  |
| limit | i32 | いいえ |  |
| q | String | いいえ |  |
| sort_by | models::PagesSortBy | いいえ |  |
| has_comments | bool | いいえ |  |

## レスポンス

返却: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## 例

[inline-code-attrs-start title = 'get_pages_public の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---