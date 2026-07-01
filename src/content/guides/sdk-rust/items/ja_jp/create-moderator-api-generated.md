## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_moderator_body | models::CreateModeratorBody | はい |  |

## レスポンス

返却: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_response.rs)

## 例

[inline-code-attrs-start title = 'create_moderator 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_moderator_body: models::CreateModeratorBody {
            email: "mod@example.com".to_string(),
            username: Some("mod_user".to_string()),
            permissions: vec!["delete".to_string(), "edit".to_string()],
            ..Default::default()
        },
    };
    let _response = create_moderator(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---