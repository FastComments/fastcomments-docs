## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| direction | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 예제

[inline-code-attrs-start title = 'postVote Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_5f6d3a2b9c1e';
const minimalResponse: VoteResponse = await postVote(commentId);

const direction: string = 'up';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiamRvZSIsImlhdCI6MTYwOTAwMDAwMH0.dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk';
const fullResponse: VoteResponse = await postVote(commentId, direction, ssoToken);
[inline-code-end]

---