## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |

## Odgovor

Vraća: `CreateV1PageReact`

## Primjer

[inline-code-attrs-start title = 'Primjer delete_v1_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(cfg: &configuration::Configuration) -> Result<(), Error> {
    let tenant_id: String = Some("acme-corp-tenant".to_string()).unwrap();
    let url_id: String = "news/article".to_string();
    let params: DeleteV1PageReactParams = DeleteV1PageReactParams {
        tenant_id,
        url_id,
        ..Default::default()
    };
    let _result = delete_v1_page_react(cfg, params).await?;
    Ok(())
}
[inline-code-end]

---