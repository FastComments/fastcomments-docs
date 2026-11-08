## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| create_hash_tag_body | models::CreateHashTagBody | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_hash_tag_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'add_hash_tag उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(cfg: &configuration::Configuration) -> Result<(), Error> {
    let params = AddHashTagParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_hash_tag_body: Some(models::CreateHashTagBody {
            tag: "news/article".to_string(),
        }),
    };
    let _response: CreateHashTagResponse = add_hash_tag(cfg, params).await?;
    Ok(())
}
[inline-code-end]