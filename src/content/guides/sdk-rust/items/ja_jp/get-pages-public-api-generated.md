テナントのページを一覧取得します。FChat デスクトップクライアントが部屋リストを構築するために使用します。  
`enableFChat` が各ページの解決されたカスタム設定で true であることが必要です。  
SSO が必要なページは、リクエスト元ユーザーのグループアクセスに基づいてフィルタリングされます。

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

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## 例

[inline-code-attrs-start title = 'get_pages_public 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]