## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_question_config_body | models::CreateQuestionConfigBody | はい |  |

## レスポンス

戻り値: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_config_200_response.rs)

## 例

[inline-code-attrs-start title = 'create_question_config の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateQuestionConfigParams = CreateQuestionConfigParams {
    tenant_id: String::from("acme-corp-tenant"),
    create_question_config_body: models::CreateQuestionConfigBody {
        key: String::from("article-usefulness"),
        label: Some(String::from("Was this article useful?")),
        description: Some(String::from("Help us improve by rating this article.")),
        required: Some(true),
        rendering_type: Some(models::QuestionRenderingType::MultipleChoice),
        custom_options: Some(vec![
            models::QuestionConfigCustomOptionsInner { value: String::from("1"), label: Some(String::from("Not useful")) },
            models::QuestionConfigCustomOptionsInner { value: String::from("3"), label: Some(String::from("Somewhat useful")) },
            models::QuestionConfigCustomOptionsInner { value: String::from("5"), label: Some(String::from("Very useful")) },
        ]),
        enabled: Some(true),
    },
};
let response: CreateQuestionConfig200Response = create_question_config(configuration, params).await?;
[inline-code-end]

---