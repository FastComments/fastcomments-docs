## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| commentId | string | Да |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetCommentBanStatusResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Primer getCommentBanStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function checkCommentBanStatus() {
  const banStatus: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654321', 'tenant_42', 'sso_token_abc123');
  const banStatusNoTenant: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654322', undefined, 'sso_token_def456');
}
[inline-code-end]