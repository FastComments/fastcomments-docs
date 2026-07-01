## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | boolean | Yes |  |
| sso | string | No |  |

## 응답

반환: [`FlagCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublicResponse.ts)

## 예제

[inline-code-attrs-start title = 'flagCommentPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoFlagComment() {
    const tenantId: string = "acme-tenant-123";
    const commentId: string = "comment-987654";
    const isFlagged: boolean = true;
    const sso: string = "sso-token-abc123";

    const result: FlagCommentPublicResponse = await flagCommentPublic(
        tenantId,
        commentId,
        isFlagged,
        sso
    );

    console.log(result);
}

demoFlagComment();
[inline-code-end]