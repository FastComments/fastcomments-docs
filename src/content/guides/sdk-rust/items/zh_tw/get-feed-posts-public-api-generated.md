req
tenantId
afterId

## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| after_id | String | 否 |  |
| limit | i32 | 否 |  |
| tags | Vec<String> | 否 |  |
| sso | String | 否 |  |
| is_crawler | bool | 否 |  |
| include_user_info | bool | 否 |  |

## 回應

回傳：[`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)