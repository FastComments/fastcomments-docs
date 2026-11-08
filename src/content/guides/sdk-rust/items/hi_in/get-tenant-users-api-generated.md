## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_tenant_users उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let _response = get_tenant_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]