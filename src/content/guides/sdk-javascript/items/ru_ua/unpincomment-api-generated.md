## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример unPinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9d2a3b';
const commentId: string = 'comment_842b9c1f';
const broadcastId: string = 'bcast_frontpage_202603';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const result: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---