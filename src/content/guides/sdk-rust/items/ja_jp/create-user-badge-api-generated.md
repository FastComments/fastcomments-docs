## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_user_badge_params | models::CreateUserBadgeParams | はい |  |

## レスポンス

戻り値: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_user_badge_200_response.rs)

## 例

[inline-code-attrs-start title = 'create_user_badge の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateUserBadgeParams = CreateUserBadgeParams {
        tenant_id: "acme-corp-tenant".into(),
        create_user_badge_params: models::CreateUserBadgeParams {
            name: "Top Contributor".into(),
            description: Some("Awarded for sustained high-quality contributions".into()),
            image_url: Some("https://cdn.acme.com/badges/top-contributor.png".into()),
            criteria: Some("100 upvotes across articles and comments".into()),
            is_active: Some(true),
            tags: Some(vec!["community".into(), "milestone".into()]),
        },
    };
    let response: CreateUserBadge200Response = create_user_badge(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---