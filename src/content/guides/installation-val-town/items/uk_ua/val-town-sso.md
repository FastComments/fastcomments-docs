Якщо ваш val уже знає, хто такий відвідувач, Secure SSO передає цю ідентичність віджету, тож користувач ніколи не побачить повторний вхід. Не потрібно створювати жодних ендпоінтів і нічого викликати під час виконання: ви обчислюєте три значення на сервері та передаєте їх у конфігурацію віджета.

Val Town пропонує вхід без налаштувань через `std/oauth`, тож відвідувач може увійти за допомогою свого облікового запису Val Town. Замініть це на те, що використовує ваш додаток; частина FastComments не змінюється.

## Формування payload на сервері

API secret підписує payload і ніколи не повинен потрапляти в код браузера. Встановіть SDK з npm, який працює на Deno-рантаймі Val Town без змін:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id must be stable for the same person, or they get a new comment identity on every login.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email is required and must be unique.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username is required and cannot be an email.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` повертає `{ userDataJSONBase64, verificationHash, timestamp }`. Саме ці три значення потрапляють у браузер. Секрет підписує їх, а потім відкидається, тож нічого на сторінці не дозволить зловмиснику підробити іншого користувача.

## Передача у віджет

[inline-code-attrs-start title = 'Widget config with SSO'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { getOAuthUserData, oauthMiddleware } from "https://esm.town/v/std/oauth/middleware.ts";

app.get("/", async (c) => {
  const session = await getOAuthUserData(c.req.raw);
  const user = session?.user;

  const config = {
    tenantId: TENANT_ID,
    urlId: "my-thread",
    ...(user
      ? { sso: { ...buildSSOPayload(user), logoutURL: "/logout" } }
      : { sso: { loginURL: "/auth/login" } }),
  };

  // ...render the widget with this config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` додає `GET /auth/login`, `GET /auth/callback` та `POST /auth/logout` за вас. Зверніть увагу, що logout — це **POST**, тоді як віджет переходить на `logoutURL` через GET, тому вкажіть `logoutURL` на невеликий власний маршрут, який надсилає цей POST.

Коли відвідувач не авторизований, передайте `sso` лише з `loginURL`. Тоді віджет покаже запрошення до входу замість анонімного поля коментарів.

## Типові проблеми

`timestamp` — це час у форматі epoch у **мілісекундах**, він не повинен бути в майбутньому і не повинен бути старшим за два дні. Генеруйте його на сервері в тому самому запиті, де обчислюється хеш. Генерація в браузері — класична помилка: значення відрізняється від того, що було підписано хешем, і кожен коментар відхиляється.

Ніколи не встановлюйте `isAdmin` або `isModerator` на основі постачальника ідентичності. Вхід через обліковий запис Val Town нічого не говорить про те, хто має модерувати ваш сайт.

Повний список полів, треди з обмеженням за групами та бейджі дивіться в [посібнику з SSO](/guide-customizations-and-configuration.html#sso-secure).