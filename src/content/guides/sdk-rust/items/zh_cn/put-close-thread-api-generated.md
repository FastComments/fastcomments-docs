---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| url_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'put_close_thread 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn close_thread() -> Result<(), Error> {
    let params: PutCloseThreadParams = PutCloseThreadParams {
        url_id: String::from("news/2026/07/acme-launch-coverage"),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature")),
    };
    let response: ApiEmptyResponse = put_close_thread(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---