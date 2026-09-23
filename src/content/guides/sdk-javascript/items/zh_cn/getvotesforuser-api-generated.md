## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回：[`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse.ts)

## 示例

[inline-code-attrs-start title = 'getVotesForUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "post_9876";
  const userId: string = "user_abcde";
  const anonUserId: string = "anon_5678";

  const responseWithUser: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, userId);
  const responseWithAnon: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
}

demo();
[inline-code-end]

---