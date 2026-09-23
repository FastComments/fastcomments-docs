## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Отговор

Връща: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Пример

[inline-code-attrs-start title = 'patchHashTag Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Извикване без незадължително тяло
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Подготовка на тяло за актуализация
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]