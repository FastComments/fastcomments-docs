## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_question_result_body | models::UpdateQuestionResultBody | Yes |  |

## Réponse

Retourne: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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