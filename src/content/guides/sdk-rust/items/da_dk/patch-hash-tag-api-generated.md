## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tag | String | Ja |  |
| tenant_id | String | Nej |  |
| update_hash_tag_body | models::UpdateHashTagBody | Nej |  |

## Svar

Returnerer: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## Eksempel

[inline-code-attrs-start title = 'patch_hash_tag Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let cfg: &configuration::Configuration = &configuration;
let body: models::UpdateHashTagBody = Default::default();
let params: PatchHashTagParams = PatchHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    update_hash_tag_body: Some(body),
};
let response: UpdateHashTagResponse = patch_hash_tag(cfg, params).await?;
[inline-code-end]

---