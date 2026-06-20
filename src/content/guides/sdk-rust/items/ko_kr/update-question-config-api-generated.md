## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_question_config_body | models::UpdateQuestionConfigBody | 예 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'update_question_config 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UpdateQuestionConfigParams = UpdateQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-config-789".to_string(),
        update_question_config_body: models::UpdateQuestionConfigBody {
            label: Some("Article feedback".to_string()),
            enabled: Some(true),
            require_login: Some(false),
            custom_options: Some(vec![
                models::QuestionConfigCustomOptionsInner {
                    key: "category".to_string(),
                    value: "news".to_string(),
                },
            ]),
        },
    };

    let _response: ApiEmptyResponse = update_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---