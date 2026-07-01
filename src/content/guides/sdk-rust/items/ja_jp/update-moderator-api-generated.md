## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| update_moderator_body | models::UpdateModeratorBody | はい |  |

## レスポンス

返り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'update_moderator の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = UpdateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-42".to_string(),
        update_moderator_body: models::UpdateModeratorBody {
            name: Some("Alice Smith".to_string()),
            email: Some("alice.smith@example.com".to_string()),
            is_active: Some(true),
        },
    };
    update_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]