## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| tag | string | Так |  |
| updateHashTagBody | UpdateHashTagBody | Ні |  |

## Відповідь

Повертає: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Виклик без необов'язкового тіла
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Підготовка тіла для оновлення
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]