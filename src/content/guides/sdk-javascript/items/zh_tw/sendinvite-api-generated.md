## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| fromName | string | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'sendInvite 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_acme_42";
  const id: string = "cmt_8f3b21";
  const fromName: string = "Ava Thompson";
  const inviteResult: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
  console.log(inviteResult);
})();
[inline-code-end]

---