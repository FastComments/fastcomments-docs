## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|------------|------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Відповідь

Повертає: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getDomainConfig Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main() {
  const tenantId: string = "acme-corp-123";
  const domain: string = "blog.acme.com";
  const config: GetDomainConfigResponse = await getDomainConfig(tenantId, domain);
  console.log(config);
}
main();
[inline-code-end]