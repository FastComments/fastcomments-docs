req
tenantId
afterId

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| after_id | String | לא |  |
| limit | i32 | לא |  |
| tags | Vec<String> | לא |  |
| sso | String | לא |  |
| is_crawler | bool | לא |  |
| include_user_info | bool | לא |  |

## תגובה

מחזיר: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)