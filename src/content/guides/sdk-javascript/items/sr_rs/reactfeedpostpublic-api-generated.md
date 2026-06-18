## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postId | string | Да |  |
| reactBodyParams | ReactBodyParams | Да |  |
| isUndo | boolean | Не |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const postId: string = 'feedpost_78901';
const reactBodyParams: ReactBodyParams = { reaction: 'like', emoji: '👍' };
const isUndo: boolean = false;
const broadcastId: string = 'broadcast_2026_06_15_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature';

const response: ReactFeedPostPublic200Response = await reactFeedPostPublic(
  tenantId,
  postId,
  reactBodyParams,
  isUndo,
  broadcastId,
  sso
);
[inline-code-end]