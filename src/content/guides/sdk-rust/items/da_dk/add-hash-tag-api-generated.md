## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_hash_tag_body | models::CreateHashTagBody | Nej |  |

## Svar

Returnerer: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_hash_tag_response.rs)

## Eksempel

[inline-code-attrs-start title = 'add_hash_tag Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---