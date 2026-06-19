## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tag | string | Да |  |
| tenantId | string | Не |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Не |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Примјер

[inline-code-attrs-start title = 'Примјер deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_7b2f9c";
const deleteHashTagRequestBody: DeleteHashTagRequestBody = { removedBy: "ops@acme-corp.com", force: true };
const result: APIEmptyResponse = await deleteHashTag(tag, tenantId, deleteHashTagRequestBody);
[inline-code-end]

---