## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |

## Відповідь

Повертає: [`DeleteV2PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReact200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteV2PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_79021";
const urlId: string = "blog/my-first-post";
const id: string = "reaction_9f8b7c";
let includeHistory: boolean | undefined = undefined; // необов'язковий прапорець, використовується в деяких викликах

const result: DeleteV2PageReact200Response = await deleteV2PageReact(tenantId, urlId, id);
console.log(result);
[inline-code-end]

---