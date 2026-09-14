Якщо ваш val вже знає, хто є відвідувачем, Secure SSO передає цю ідентичність віджету, тому користувач ніколи не бачить другий вхід. Не потрібно створювати кінцеві точки і нічого викликати під час виконання: ви обчислюєте три значення на сервері та передаєте їх у конфігурацію віджета.

Val Town постачається з входом без налаштувань за допомогою `std/oauth`, тому відвідувач може увійти за допомогою облікового запису Val Town, який у нього вже є. Замініть це на те, що використовує ваш додаток; частина FastComments не змінюється.

## Build the payload on the server

Секрет API підписує корисне навантаження і ніколи не повинен потрапляти в код браузера. Встановіть SDK з npm, який працює у середовищі Deno від Val Town без змін:

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

`getPayload()` повертає `{ userDataJSONBase64, verificationHash, timestamp }`. Ці три значення — це все, що потрапляє в браузер. Секрет підписує їх, а потім відкидається, тому нічого на сторінці не дозволяє читачу підробити інший користувач.

## Pass it to the widget

[inline-code-attrs-start title = 'Конфігурація віджета з SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` додає `GET /auth/login`, `GET /auth/callback` та `POST /auth/logout` для вас. Зауважте, що вихід (logout) — це **POST**, тоді як віджет переходить за `logoutURL` за допомогою GET, тому вкажіть `logoutURL` на невеликий маршрут вашого додатку, який виконує POST.

Коли відвідувач виходить з системи, передайте `sso` лише з `loginURL`. Тоді віджет покаже запит на вхід замість анонімного поля коментаря.

## Things that go wrong

`timestamp` — це мілісекунди епохи, не повинен бути в майбутньому і не повинен бути старшим за два дні. Генеруйте його на сервері в тому ж запиті, де обчислюється хеш. Генерація в браузері — це класична помилка: значення відрізняється від того, що було захешовано, і кожен коментар відхиляється.

Ніколи не встановлюйте `isAdmin` або `isModerator` з провайдера ідентифікації. Вхід за допомогою облікового запису Val Town нічого не говорить про те, хто повинен модерувати ваш сайт.

Перегляньте [посібник SSO](/guide-sso.html) для повного списку полів, потоків з груповим доступом та значків.