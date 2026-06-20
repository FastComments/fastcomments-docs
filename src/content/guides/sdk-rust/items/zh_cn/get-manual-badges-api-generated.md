---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| sso | String | 否 |  |

## 响应

返回: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_manual_badges_response.rs)

## 示例

[inline-code-attrs-start title = 'get_manual_badges 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_manual_badges() -> Result<(), Error> {
    let params: GetManualBadgesParams = GetManualBadgesParams {
        sso: Some(String::from("https://sso.acme-corp.com/authorize?tenant=acme-corp-tenant")),
    };
    let response: GetTenantManualBadgesResponse = get_manual_badges(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---