## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| textSearch | string | Ні |  |
| byIPFromComment | string | Ні |  |
| filter | string | Ні |  |
| searchFilters | string | Ні |  |
| demo | boolean | Ні |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const count: GetCountResponse = await getCount({
    textSearch: "order issue",
    byIPFromComment: "198.51.100.23",
    filter: "pending",
    demo: true,
    tenantId: "acme_corp",
    sso: "sso_abcdef123456"
  });
  console.log(count);
}
main();
[inline-code-end]