## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_question_config_body | models::CreateQuestionConfigBody | Yes |  |

## Response

Returns: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_config_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_question_config Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateQuestionConfigParams = CreateQuestionConfigParams {
    tenant_id: String::from("acme-corp-tenant"),
    create_question_config_body: CreateQuestionConfigBody {
        key: String::from("article_feedback"),
        label: Some(String::from("Was this article helpful?")),
        description: Some(String::from("Quick feedback for news and editorial pieces")),
        question_type: Some(QuestionRenderingType::MultipleChoice),
        required: Some(true),
        options: Some(vec![
            QuestionConfigCustomOptionsInner { value: String::from("yes"), label: Some(String::from("Yes")) },
            QuestionConfigCustomOptionsInner { value: String::from("no"), label: Some(String::from("No")) },
            QuestionConfigCustomOptionsInner { value: String::from("unsure"), label: Some(String::from("Not sure")) },
        ]),
    },
};
let response: CreateQuestionConfig200Response = create_question_config(&configuration, params).await?;
[inline-code-end]
