## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |
| title | String | いいえ |  |

## レスポンス

戻り値: `CreateV1PageReact`

## 例

[inline-code-attrs-start title = 'create_v1_page_react の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<CreateV1PageReact, Error> {
    let params = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/06/launch".to_string(),
        title: Some("Acme Launch Coverage".to_string()),
    };
    let reaction: CreateV1PageReact = create_v1_page_react(&configuration, params).await?;
    Ok(reaction)
}
[inline-code-end]

---