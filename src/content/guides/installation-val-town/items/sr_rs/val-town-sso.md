Ако ваш val већ зна ко је посетилац, Secure SSO предаје тај идентитет виџету тако да посетилац никада не види другу пријаву. Нема крајњих тачака за изградњу и нема ничега за позивати у току рада: израчунавате три вредности на серверу и прослеђујете их у конфигурацију виџета.

Val Town нуди пријаву без конфигурације преко `std/oauth`, тако да посетилац може да се пријави Val Town налогом који већ има. Замените то оним што ваша апликација користи; FastComments део се не мења.

## Изградите payload на серверу

API тајна потписује payload и никада не сме да доспе до кода у прегледачу. Инсталирајте SDK са npm-а, који ради на Val Town Deno runtime-у какав јесте:

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

`getPayload()` враћа `{ userDataJSONBase64, verificationHash, timestamp }`. Те три вредности су све што доспева до прегледача. Тајна их потписује и затим се одбацује, тако да ништа на страни не омогућава читаоцу да фалсификује другог корисника.

## Проследите га виџету

[inline-code-attrs-start title = 'Конфигурација виџета са SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` додаје `GET /auth/login`, `GET /auth/callback` и `POST /auth/logout` уместо вас. Имајте у виду да је одјава **POST**, док виџет одлази на `logoutURL` са GET, па усмерите `logoutURL` на малу руту коју сами правите и која шаље тај POST.

Када је посетилац одјављен, проследите `sso` са само `loginURL`. Виџет тада приказује позив за пријаву уместо анонимног поља за коментаре.

## Ствари које крену наопако

`timestamp` је у епохи **милисекундама**, не сме бити у будућности и не сме бити старији од два дана. Генеришите га на серверу у истом захтеву у коме се рачуна хеш. Генерисање у прегледачу је класична грешка: вредност се разликује од оне која је хеширана и сваки коментар бива одбијен.

Никада не постављајте `isAdmin` или `isModerator` од добављача идентитета. Пријава Val Town налогом не говори ништа о томе ко би требало да модерира ваш сајт.

Погледајте [SSO водич](/guide-customizations-and-configuration.html#sso-secure) за потпуну листу поља, теме ограничене групама и значке.