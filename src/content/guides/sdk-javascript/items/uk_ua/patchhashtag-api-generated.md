## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tag | string | Так |  |
| tenantId | string | Ні |  |
| updateHashTagBody | UpdateHashTagBody | Ні |  |

## Відповідь

Повертає: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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