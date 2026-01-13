---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| skip | f64 | 否 |  |

## 回應

回傳: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_moderators 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let moderators: GetModerators200Response = get_moderators(&configuration, params).await?;
    let _moderators = moderators;
    Ok(())
}
[inline-code-end]

---