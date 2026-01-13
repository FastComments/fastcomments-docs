## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vraća: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_config_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_question_config Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_config() -> Result<(), Error> {
    let tenant_env: Option<&str> = Some("prod");
    let tenant_id: String = match tenant_env {
        Some(env) => format!("acme-corp-tenant-{}", env),
        None => "acme-corp-tenant".to_string(),
    };
    let params: GetQuestionConfigParams = GetQuestionConfigParams {
        tenant_id,
        id: "news/article/2026/01/12-politics".to_string(),
    };
    let response: GetQuestionConfig200Response = get_question_config(&configuration, params).await?;
    let _status: ApiStatus = response.0;
    Ok(())
}
[inline-code-end]

---