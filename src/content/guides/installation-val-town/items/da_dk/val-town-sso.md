Hvis din val allerede ved, hvem den besøgende er, giver Secure SSO denne identitet videre til widget'en, så de aldrig ser en anden login. Der er ingen endpoints at bygge og intet at kalde ved runtime: du beregner tre værdier server-side og sender dem med i widget-konfigurationen.

Val Town leverer zero-config login med `std/oauth`, så den besøgende kan logge ind med den Val Town-konto, de allerede har. Udskift det med, hvad din app nu bruger; FastComments-delen ændres ikke.

## Byg payload'en på serveren

API-secret'en signerer payload'en og må aldrig nå browser-koden. Installer SDK'en fra npm, som virker på Val Towns Deno-runtime som den er:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id skal være stabilt for den samme person, ellers får de en ny kommentar-identitet ved hvert login.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email er påkrævet og skal være unik.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username er påkrævet og må ikke være en email.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` returnerer `{ userDataJSONBase64, verificationHash, timestamp }`. Det er de tre værdier, der når browseren. Secret'en signerer dem og smides derefter væk, så intet på siden gør det muligt for en læser at forfalske en anden bruger.

## Send det til widget'en

[inline-code-attrs-start title = 'Widget-konfiguration med SSO'; type='javascript' inline-code-attrs-end]
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

  // ...render widget'en med denne konfiguration
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` tilføjer `GET /auth/login`, `GET /auth/callback` og `POST /auth/logout` for dig. Bemærk, at logout er en **POST**, mens widget'en navigerer til `logoutURL` med en GET, så peg `logoutURL` på en lille route hos dig selv, som sender POST'en.

Når den besøgende er logget ud, sendes `sso` med kun en `loginURL`. Widget'en viser så en login-prompt i stedet for en anonym kommentarboks.

## Ting der går galt

`timestamp` er i epoch-**millisekunder**, må ikke ligge i fremtiden og må ikke være mere end to dage gammel. Generér den på serveren i samme request, som beregner hashen. At generere den i browseren er den klassiske fejl: værdien afviger fra den, der blev hashet, og hver kommentar afvises.

Sæt aldrig `isAdmin` eller `isModerator` ud fra identitetsudbyderen. At logge ind med en Val Town-konto siger intet om, hvem der bør moderere dit site.

Se [SSO-guiden](/guide-customizations-and-configuration.html#sso-secure) for den fulde feltliste, gruppebegrænsede tråde og badges.