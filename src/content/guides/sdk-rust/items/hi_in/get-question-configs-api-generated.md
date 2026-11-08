## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_configs_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_question_configs उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetQuestionConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let _response: GetQuestionConfigsResponse = get_question_configs(&configuration, params).await?;
    Ok(())
}
[inline-code-end]