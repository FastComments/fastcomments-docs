## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| update_question_result_body | models::UpdateQuestionResultBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_question_result उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UpdateQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-9876".to_string(),
        update_question_result_body: models::UpdateQuestionResultBody {
            status: Some("approved".to_string()),
            score: Some(95),
            ..Default::default()
        },
    };
    let _ = update_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]