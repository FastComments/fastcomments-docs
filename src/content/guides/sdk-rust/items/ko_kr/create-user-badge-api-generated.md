## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| create_user_badge_params | models::CreateUserBadgeParams | 예 |  |

## 응답

반환: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_user_badge_200_response.rs)

## 예제

[inline-code-attrs-start title = 'create_user_badge 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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