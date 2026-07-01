## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getManualBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_ABCdef123456";

  // Виклик з обома необов'язковими параметрами
  const responseFull: GetManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  console.log(responseFull);

  // Виклик лише з tenantId
  const responseTenantOnly: GetManualBadgesResponse = await getManualBadges(tenantId);
  console.log(responseTenantOnly);
}

fetchBadges();
[inline-code-end]