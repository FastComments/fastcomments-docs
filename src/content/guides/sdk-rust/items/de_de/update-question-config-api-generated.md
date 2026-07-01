## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| update_question_config_body | models::UpdateQuestionConfigBody | Ja |  |

## Antwort

Rückgabe: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Beispiel

[inline-code-attrs-start title = 'update_question_config Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UpdateQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        update_question_config_body: UpdateQuestionConfigBody {
            title: "Breaking News".to_string(),
            is_active: true,
            custom_options: vec![
                QuestionConfigCustomOptionsInner {
                    key: "priority".to_string(),
                    value: "high".to_string(),
                },
            ],
            description: Some("Config for breaking news article".to_string()),
        },
    };
    update_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]