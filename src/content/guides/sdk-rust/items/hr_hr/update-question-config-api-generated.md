## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_question_config_body | models::UpdateQuestionConfigBody | Da |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'update_question_config Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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