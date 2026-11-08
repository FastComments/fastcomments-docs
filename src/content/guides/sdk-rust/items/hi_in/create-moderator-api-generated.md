## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| create_moderator_body | models::CreateModeratorBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'create_moderator उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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