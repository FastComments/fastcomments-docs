## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |

## Resposta

Retorna: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_response.rs)

## Exemplo

[inline-code-attrs-start title = 'get_question_result Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_result(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-12345".to_string(),
        locale: Some("en-US".to_string()),
    };
    let _response: GetQuestionResultResponse = get_question_result(config, params).await?;
    Ok(())
}
[inline-code-end]

---