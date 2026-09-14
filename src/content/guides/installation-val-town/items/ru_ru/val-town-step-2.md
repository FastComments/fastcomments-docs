`tenantId: "demo"` — это общедоступный публичный песочница. Она работает без регистрации, поэтому примеры используют её, но все остальные, использующие FastComments, пишут в одних и тех же ветках, и любой может их модерировать. Переключите её перед публикацией чего‑то важного.

Ваш tenant ID находится на [странице API‑секрета](https://fastcomments.com/auth/my-account/api-secret).

Tenant ID является публичным и должен находиться в коде браузера. API‑секрет — нет, и ничего на этой странице не требует его.

## Считать из переменной окружения

Val Town vals публичны в бесплатном тарифе, поэтому их исходный код доступен всем. Храните любые конфиденциальные данные в переменных окружения, считывайте их с помощью `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Это имеет большее значение на Val Town по второй причине: **при ремиксе val копируются ключи переменных окружения, но не их значения.** Секрет, хранящийся в переменной окружения, не переходит вместе с вашим val в аккаунт другого пользователя. Секрет, записанный в файл, переходит.

Возврат к значению `"demo"` позволяет val работать для любого, кто ремиксит его до установки собственного tenant.

## EU‑аккаунты

Аккаунт, его данные и ключи находятся в одном регионе. Если ваш аккаунт был создан на `eu.fastcomments.com`, каждая конфигурация виджета также должна содержать `region: "eu"`, а скрипты загружаются с `cdn-eu.fastcomments.com`. В противном случае оставьте оба параметра без изменений.

---