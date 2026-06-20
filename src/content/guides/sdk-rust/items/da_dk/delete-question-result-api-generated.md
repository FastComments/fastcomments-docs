## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Returnerer: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på delete_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let id: String = "news/article-12345/question-67890".to_string();

    let params = DeleteQuestionResultParams {
        tenant_id,
        id,
    };

    let response: ApiEmptyResponse = delete_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---