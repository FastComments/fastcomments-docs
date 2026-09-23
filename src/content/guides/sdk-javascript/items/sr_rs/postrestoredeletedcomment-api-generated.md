## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'postRestoreDeletedComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const resultRequired: APIEmptyResponse = await postRestoreDeletedComment(tenantId, commentId);

const broadcastId: string = "brd_001";
const sso: string = "sso_token_abc123";

const resultAll: APIEmptyResponse = await postRestoreDeletedComment(
  tenantId,
  commentId,
  broadcastId,
  sso
);
[inline-code-end]

---