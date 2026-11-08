## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_question_result उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_result(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-12345".to_string(),
        locale: Some("en-US".to_string()),
    };
    let _response: GetQuestionResultResponse = get_question_result(config, params).await?;
    Ok(())
}
[inline-code-end]