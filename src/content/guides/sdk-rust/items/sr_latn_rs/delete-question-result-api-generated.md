## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'delete_question_result Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteQuestionResultParams {
        tenant_id: "acme-corp".to_string(),
        id: "question-9876".to_string(),
    };
    delete_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]