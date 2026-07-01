## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`GetCommentChildrenResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentChildrenResponse.ts)

## 예시

[inline-code-attrs-start title = 'getCommentChildren 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const tenantId: string = "tenant_xyz";
const sso: string = "sso_987654";

const fullResponse: GetCommentChildrenResponse = await getCommentChildren(commentId, tenantId, sso);
const minimalResponse: GetCommentChildrenResponse = await getCommentChildren(commentId);
[inline-code-end]