## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| direction | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 示例

[inline-code-attrs-start title = 'postVote 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_5f6d3a2b9c1e';
const minimalResponse: VoteResponse = await postVote(commentId);

const direction: string = 'up';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiamRvZSIsImlhdCI6MTYwOTAwMDAwMH0.dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk';
const fullResponse: VoteResponse = await postVote(commentId, direction, ssoToken);
[inline-code-end]

---