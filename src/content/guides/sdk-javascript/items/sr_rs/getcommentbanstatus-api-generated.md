## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Одговор

Враћа: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getCommentBanStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_987654";
  const ssoToken: string = "sso_user_abc";

  const statusWithSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId, ssoToken);
  const statusWithoutSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId);
})();
[inline-code-end]

---