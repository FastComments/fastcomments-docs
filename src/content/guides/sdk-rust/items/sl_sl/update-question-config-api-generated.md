## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_question_config_body | models::UpdateQuestionConfigBody | Da |  |

## Odgovor

Vrne: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'update_question_config Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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