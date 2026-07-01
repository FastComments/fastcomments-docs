## 파라미터

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| title | String | 아니오 |  |

## 응답

반환: `CreateV1PageReact`

## 예시

[inline-code-attrs-start title = 'create_v1_page_react 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateV1PageReactParams = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        title: Some("Rust Community Update".to_string()),
    };
    let _response = create_v1_page_react(&config, params).await?;
    Ok(())
}
[inline-code-end]