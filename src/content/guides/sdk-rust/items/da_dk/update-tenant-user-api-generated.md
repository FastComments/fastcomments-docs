## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Ja |  |
| update_comments | String | Nej |  |

## Respons

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'update_tenant_user Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_user_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: UpdateTenantUserParams = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-78b2".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            username: "jdoe".to_string(),
            display_name: "John Doe".to_string(),
            email: "john.doe@acme.com".to_string(),
            roles: vec!["moderator".to_string()],
            suspended: false,
        },
        update_comments: Some("Promoted to moderator for community moderation".to_string()),
    };
    let response: FlagCommentPublic200Response = update_tenant_user(configuration, params).await?;
    println!("updated user response status: {:?}", response);
    Ok(())
}
[inline-code-end]

---