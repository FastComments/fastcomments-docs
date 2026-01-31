## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_question_config_body | models::UpdateQuestionConfigBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_question_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UpdateQuestionConfigParams = UpdateQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-comments".to_string(),
        update_question_config_body: models::UpdateQuestionConfigBody {
            question_text: Some("Was this article helpful?".to_string()),
            help_text: Some("Select the option that best describes your experience.".to_string()),
            required: Some(true),
            rendering_type: Some(models::QuestionRenderingType::Inline),
            custom_options: Some(vec![
                models::QuestionConfigCustomOptionsInner { value: "yes".to_string(), label: "Yes".to_string() },
                models::QuestionConfigCustomOptionsInner { value: "no".to_string(), label: "No".to_string() },
            ]),
        },
    };
    let response: FlagCommentPublic200Response = update_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
