## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |

## Réponse

Renvoie : `GetV2PageReacts`

## Exemple

[inline-code-attrs-start title = 'get_v2_page_reacts Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetV2PageReactsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        page: Some(1),
        page_size: Some(50),
    };
    let _reacts = get_v2_page_reacts(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---