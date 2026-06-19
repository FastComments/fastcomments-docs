## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tag | string | Да |  |
| tenantId | string | Не |  |
| updateHashTagBody | UpdateHashTagBody | Не |  |

## Отговор

Връща: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Пример

[inline-code-attrs-start title = 'patchHashTag Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_8421";
const updateHashTagBody: UpdateHashTagBody = {
  name: "Release Notes",
  description: "Thread for discussing feature releases and changelogs",
  isActive: true
};
const result: UpdateHashTagResponse = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---