## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| tag | String | हाँ |  |
| update_hash_tag_body | models::UpdateHashTagBody | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'patch_hash_tag उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PatchHashTagParams {
        tenant_id: "acme-corp-tenant".into(),
        tag: "news/article".into(),
        update_hash_tag_body: Some(models::UpdateHashTagBody::default()),
    };
    let _response = patch_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]