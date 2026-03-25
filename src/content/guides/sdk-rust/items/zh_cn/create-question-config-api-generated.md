---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_question_config_body | models::CreateQuestionConfigBody | 是 |  |

## 响应

返回: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_config_200_response.rs)

## 示例

[inline-code-attrs-start title = 'create_question_config 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateQuestionConfigParams = CreateQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_question_config_body: models::CreateQuestionConfigBody {
            slug: "news/article".to_string(),
            title: "Reader satisfaction".to_string(),
            description: Some("How satisfied are you with this article?".to_string()),
            required: Some(false),
            rendering_type: Some(QuestionRenderingType::Inline),
            custom_options: Some(vec![
                models::QuestionConfigCustomOptionsInner { value: "very_satisfied".to_string(), label: Some("Very satisfied".to_string()) },
                models::QuestionConfigCustomOptionsInner { value: "unsatisfied".to_string(), label: Some("Unsatisfied".to_string()) },
            ]),
        },
    };
    let response: CreateQuestionConfig200Response = create_question_config(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---