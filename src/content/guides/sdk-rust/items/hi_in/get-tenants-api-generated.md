## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| meta | String | नहीं |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_tenants उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetTenantsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        meta: Some("news/article".to_string()),
        skip: Some(10.0),
    };
    let _response = get_tenants(config, params).await?;
    Ok(())
}
[inline-code-end]