## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_configs_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_question_configs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetQuestionConfigsParams = GetQuestionConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let response: GetQuestionConfigs200Response = get_question_configs(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---