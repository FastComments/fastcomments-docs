## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| create_hash_tag_body | models::CreateHashTagBody | No |  |

## Risposta

Restituisce: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_hash_tag_response.rs)

## Esempio

[inline-code-attrs-start title = 'add_hash_tag Esempio'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(cfg: &configuration::Configuration) -> Result<(), Error> {
    let params = AddHashTagParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_hash_tag_body: Some(models::CreateHashTagBody {
            tag: "news/article".to_string(),
        }),
    };
    let _response: CreateHashTagResponse = add_hash_tag(cfg, params).await?;
    Ok(())
}
[inline-code-end]