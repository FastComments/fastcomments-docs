---
## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_question_config_body | models::CreateQuestionConfigBody | はい |  |

## レスポンス

戻り値: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_config_response.rs)

## 例

[inline-code-attrs-start title = 'create_question_config の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateQuestionConfigParams = CreateQuestionConfigParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_question_config_body: models::CreateQuestionConfigBody {
        slug: "news/article".to_string(),
        title: "Article Comments".to_string(),
        description: Some("Questions configuration for news articles".to_string()),
        enabled: Some(true),
        allow_anonymous: Some(false),
        moderation_level: Some("pre_moderation".to_string()),
        custom_options: Some(vec![
            models::QuestionConfigCustomOptionsInner { key: "max_length".to_string(), value: "500".to_string() }
        ]),
    },
};
let response: CreateQuestionConfigResponse = create_question_config(&configuration, params).await?;
[inline-code-end]

---