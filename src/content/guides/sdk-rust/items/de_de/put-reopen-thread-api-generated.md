## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| url_id | String | Ja |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Beispiel

[inline-code-attrs-start title = 'put_reopen_thread Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reopen_thread() -> Result<(), Error> {
    let params: PutReopenThreadParams = PutReopenThreadParams {
        url_id: String::from("acme-corp/news/article-2026-06-19"),
        sso: Some(String::from("sso-token-9f8e7d6c")),
    };
    let response: ApiEmptyResponse = put_reopen_thread(configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---