Ако вашият val вече знае кой е посетителят, Secure SSO предава тази идентичност на уиджета, така че той никога да не вижда второ влизане. Няма нужда от създаване на крайни точки и нищо не се извиква по време на изпълнение: изчислявате три стойности на сървъра и ги предавате в конфигурацията на уиджета.

Val Town предоставя вход без конфигурация с `std/oauth`, така че посетителят може да влезе с Val Town акаунта, който вече притежава. Заменете това с каквото вашето приложение използва; частта на FastComments остава непроменена.

## Създаване на полезния товар на сървъра

Тайната на API подписва полезния товар и никога не трябва да достигне до кода в браузъра. Инсталирайте SDK от npm, което работи директно в Deno средата на Val Town:

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

`getPayload()` връща `{ userDataJSONBase64, verificationHash, timestamp }`. Тези три стойности са всичко, което достига до браузъра. Тайната ги подписва и след това се изхвърля, така че нищо в страницата не позволява на читателя да подправи различен потребител.

## Предайте го на уиджета

[inline-code-attrs-start title = 'Конфигурация на уиджет с SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` добавя `GET /auth/login`, `GET /auth/callback` и `POST /auth/logout` за вас. Обърнете внимание, че изходът е **POST**, докато уиджетът навигира към `logoutURL` с GET, затова насочете `logoutURL` към малка ваша рута, която изпраща POST заявка.

Когато посетителят е излязъл, предайте `sso` само с `loginURL`. Уиджетът тогава показва прозорец за вход вместо анонимен кутия за коментари.

## Неочаквани проблеми

`timestamp` е епоха в **милисекунди**, не трябва да бъде в бъдещето и не трябва да е по-стар от два дни. Генерирайте го на сървъра в същата заявка, която изчислява хеша. Генерирането му в браузъра е класическа грешка: стойността се различава от тази, която е била хеширана, и всеки коментар се отхвърля.

Никога не задавайте `isAdmin` или `isModerator` от доставчика на идентичност. Влизането с Val Town акаунт не казва нищо за това кой трябва да модерира вашия сайт.

Вижте [ръководството за SSO](/guide-sso.html) за пълен списък на полетата, нишки с групово ограничение и значки.