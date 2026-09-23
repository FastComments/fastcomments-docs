## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| createHashTagBody | CreateHashTagBody | Нет |  |

## Ответ

Возвращает: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateHashTagResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример addHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c3f5e8b2-9d4a-4f6a-8b2c-1a2b3c4d5e6f";

const tagPayload: CreateHashTagBody = {
  tag: "typescript",
  description: "Discussions about TypeScript"
};

const responseWithBody: CreateHashTagResponse = await addHashTag(tenantId, tagPayload);
const responseWithoutBody: CreateHashTagResponse = await addHashTag(tenantId);
[inline-code-end]