## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tag | String | Ja |  |
| tenant_id | String | Nee |  |
| update_hash_tag_body | models::UpdateHashTagBody | Nee |  |

## Response

Retourneert: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_hash_tag_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van patch_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PatchHashTagParams = PatchHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    update_hash_tag_body: Some(models::UpdateHashTagBody {
        label: Some("World News".to_string()),
        description: Some("Articles related to world events.".to_string()),
        enabled: Some(true),
    }),
};

let response: PatchHashTag200Response = patch_hash_tag(&configuration, params).await?
[inline-code-end]

---