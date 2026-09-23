## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentIds | string | はい |  |
| sso | string | いいえ |  |

## 応答

返却: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckBlockedCommentsResponse.ts)

## 例

[inline-code-attrs-start title = 'checkedCommentsForBlocked の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "tenant_12345";
    const commentIds: string = "cmt_9876,cmt_5432";
    const ssoToken: string = "sso_user_abc123";

    const responseWithSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
    const responseWithoutSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds);
})();
[inline-code-end]