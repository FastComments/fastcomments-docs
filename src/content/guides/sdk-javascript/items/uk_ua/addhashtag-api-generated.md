## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Ні |  |
| createHashTagBody | CreateHashTagBody | Ні |  |

## Відповідь

Повертає: [`AddHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagResponse.ts)

## Приклад

[inline-code-attrs-start title = 'addHashTag Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const newHashTag: CreateHashTagBody = {
  tag: "typescript",
  description: "Discussions about TypeScript"
};

const responseWithTenant: AddHashTagResponse = await addHashTag(tenantId, newHashTag);

const responseWithoutTenant: AddHashTagResponse = await addHashTag(undefined, newHashTag);
[inline-code-end]