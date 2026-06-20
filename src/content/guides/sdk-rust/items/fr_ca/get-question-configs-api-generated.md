## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| skip | f64 | Non |  |

## Réponse

Renvoie : [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_configs_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_question_configs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_configs() -> Result<GetQuestionConfigsResponse, Error> {
    let params: GetQuestionConfigsParams = GetQuestionConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetQuestionConfigsResponse = get_question_configs(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---