## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| tag | String | Yes |  |
| update_hash_tag_body | models::UpdateHashTagBody | No |  |

## Odgovor

Vraća: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## Primjer

[inline-code-attrs-start title = 'patch_hash_tag Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PatchHashTagParams {
        tenant_id: "acme-corp-tenant".into(),
        tag: "news/article".into(),
        update_hash_tag_body: Some(models::UpdateHashTagBody::default()),
    };
    let _response = patch_hash_tag(&configuration, params).await?;
    Ok(())
}
[inline-code-end]