## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Não |  |
| tenantId | string | Não |  |
| urlId | string | Não |  |
| page | number | Não |  |
| direction | SortDirections | Não |  |
| lastGenDate | number | Não |  |
| repliesToUserId | string | Não |  |
| fetchPageForCommentId | string | Não |  |
| includei10n | boolean | Não |  |
| useFullTranslationIds | boolean | Não |  |
| locale | string | Não |  |
| includeConfig | boolean | Não |  |
| includeNotificationCount | boolean | Não |  |
| countAll | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = 'user_82f9b';
const tenantId: string = 'tenant_22';
const page: number = 2;
const lastGenDate: number = Date.now();
const includei10n: boolean = true;
const useFullTranslationIds: boolean = false;
const locale: string = 'en-US';
const includeConfig: boolean = true;
const includeNotificationCount: boolean = true;
const countAll: boolean = false;
const sso: string = 'sso-token-1a2b';
const commentsResponse: GetCommentsForUserResponse = await getCommentsForUser(userId, tenantId, undefined, page, undefined, lastGenDate, undefined, undefined, includei10n, useFullTranslationIds, locale, includeConfig, includeNotificationCount, countAll, sso);
[inline-code-end]

---