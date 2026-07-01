## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## 응답

반환: [`GetCommentTextResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentTextResponse1.ts)

## 예시

[inline-code-attrs-start title = 'getCommentText 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp-tenant";
  const commentId: string = "cmt-5f2e9a1b";
  const editKey: string = "edk-9b7c3";
  const ssoToken: string = "sso-xyz789";

  const commentOnly: GetCommentTextResponse1 = await getCommentText(tenantId, commentId);
  const commentWithEdit: GetCommentTextResponse1 = await getCommentText(tenantId, commentId, editKey);
  const commentFull: GetCommentTextResponse1 = await getCommentText(tenantId, commentId, editKey, ssoToken);
}
run();
[inline-code-end]

---