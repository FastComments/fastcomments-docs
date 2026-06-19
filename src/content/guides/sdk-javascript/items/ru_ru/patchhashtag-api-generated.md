## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Да |  |
| tenantId | string | Нет |  |
| updateHashTagBody | UpdateHashTagBody | Нет |  |

## Ответ

Возвращает: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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