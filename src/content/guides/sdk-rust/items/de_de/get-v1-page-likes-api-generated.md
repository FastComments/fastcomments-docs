---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |

## Antwort

Gibt zurück: `GetV1PageLikes`

## Beispiel

[inline-code-attrs-start title = 'get_v1_page_likes Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_page_likes() -> Result<(), Error> {
    let params: GetV1PageLikesParams = GetV1PageLikesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-123".to_string(),
    };
    let optional_referrer: Option<String> = Some("https://news.example.com/article-123".to_string());
    let likes: GetV1PageLikes = get_v1_page_likes(&configuration, params).await?;
    println!("retrieved page likes: {:?}", optional_referrer);
    let _consumed: GetV1PageLikes = likes;
    Ok(())
}
[inline-code-end]

---