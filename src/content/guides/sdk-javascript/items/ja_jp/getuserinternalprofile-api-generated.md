## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | いいえ |  |
| tenantId | string | いいえ |  |
| sso | string | いいえ |  |

## 応答

返却: [`GetUserInternalProfileResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse1.ts)

## 例

[inline-code-attrs-start title = 'getUserInternalProfile 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const fullProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_12345",
    tenantId: "tenant_67890",
    sso: "sso_token_abcdef"
  });

  const partialProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_98765"
  });
})();
[inline-code-end]