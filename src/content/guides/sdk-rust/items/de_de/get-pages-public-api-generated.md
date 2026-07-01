List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.  
Seiten, die SSO erfordern, werden anhand des Gruppen‑Zugriffs des anfordernden Benutzers gefiltert.

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| cursor | String | Nein |  |
| limit | i32 | Nein |  |
| q | String | Nein |  |
| sort_by | models::PagesSortBy | Nein |  |
| has_comments | bool | Nein |  |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Example

[inline-code-attrs-start title = 'get_pages_public Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]