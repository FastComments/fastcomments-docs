## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| fromName | string | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Примјер

[inline-code-attrs-start title = 'sendInvite Примјер'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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