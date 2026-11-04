Als je val al weet wie de bezoeker is, geeft Secure SSO die identiteit door aan de widget zodat deze nooit een tweede login te zien krijgt. Er zijn geen endpoints om te bouwen en niets om tijdens runtime aan te roepen: je berekent drie waarden server-side en geeft ze door in de widget-config.

Val Town levert zero-config login mee met `std/oauth`, zodat de bezoeker kan inloggen met het Val Town-account dat hij al heeft. Vervang dat door wat jouw app gebruikt; het FastComments-gedeelte verandert niet.

## Bouw de payload op de server

De API secret ondertekent de payload en mag nooit in browsercode terechtkomen. Installeer de SDK vanaf npm, die op Val Town's Deno-runtime direct werkt:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id moet stabiel zijn voor dezelfde persoon, anders krijgt die bij elke login een nieuwe comment-identiteit.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email is verplicht en moet uniek zijn.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username is verplicht en mag geen e-mailadres zijn.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` geeft `{ userDataJSONBase64, verificationHash, timestamp }` terug. Die drie waarden zijn alles wat de browser bereikt. De secret ondertekent ze en wordt daarna weggegooid, dus niets op de pagina stelt een lezer in staat om een andere gebruiker te vervalsen.

## Geef het door aan de widget

[inline-code-attrs-start title = 'Widget config met SSO'; type='javascript' inline-code-attrs-end]
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

  // ...render de widget met deze config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` voegt `GET /auth/login`, `GET /auth/callback` en `POST /auth/logout` voor je toe. Let op: logout is een **POST**, terwijl de widget naar `logoutURL` navigeert met een GET, dus laat `logoutURL` wijzen naar een eigen kleine route die de POST verstuurt.

Wanneer de bezoeker is uitgelogd, geef je `sso` door met alleen een `loginURL`. De widget toont dan een login-prompt in plaats van een anoniem commentaarvak.

## Dingen die misgaan

`timestamp` is epoch in **milliseconden**, mag niet in de toekomst liggen en mag niet ouder zijn dan twee dagen. Genereer het op de server in dezelfde request waarin de hash wordt berekend. Het in de browser genereren is de klassieke fout: de waarde verschilt van degene die gehasht is en elk commentaar wordt geweigerd.

Zet `isAdmin` of `isModerator` nooit op basis van de identity provider. Inloggen met een Val Town-account zegt niets over wie jouw site moet modereren.

Zie de [SSO-gids](/guide-customizations-and-configuration.html#sso-secure) voor de volledige lijst met velden, threads met groepstoegang en badges.