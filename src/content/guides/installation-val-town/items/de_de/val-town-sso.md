If Ihr Val bereits weiß, wer der Besucher ist, übergibt Secure SSO diese Identität an das Widget, sodass sie nie einen zweiten Login sehen. Es gibt keine Endpunkte zu erstellen und nichts, das zur Laufzeit aufgerufen werden muss: Sie berechnen drei Werte serverseitig und übergeben sie in der Widget‑Konfiguration.

Val Town liefert eine Null‑Konfigurations‑Anmeldung mit `std/oauth`, sodass der Besucher sich mit dem Val Town‑Konto anmelden kann, das er bereits hat. Ersetzen Sie dies durch das, was Ihre App verwendet; der FastComments‑Teil ändert sich nicht.

## Build the payload on the server

The API secret signs the payload and must never reach browser code. Install the SDK from npm, which works on Val Town's Deno runtime as‑is:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id muss für dieselbe Person stabil sein, sonst erhalten sie bei jedem Login eine neue Kommentaridentität.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // E‑Mail ist erforderlich und muss eindeutig sein.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // Benutzername ist erforderlich und darf keine E‑Mail sein.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` gibt `{ userDataJSONBase64, verificationHash, timestamp }` zurück. Diese drei Werte sind alles, was den Browser erreicht. Das Geheimnis signiert sie und wird dann verworfen, sodass nichts auf der Seite einem Leser ermöglicht, einen anderen Benutzer zu fälschen.

## Pass it to the widget

[inline-code-attrs-start title = 'Widget-Konfiguration mit SSO'; type='javascript' inline-code-attrs-end]
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

  // ...rendern Sie das Widget mit dieser Konfiguration
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` fügt `GET /auth/login`, `GET /auth/callback` und `POST /auth/logout` für Sie hinzu. Beachten Sie, dass Logout ein **POST** ist, während das Widget zu `logoutURL` mit einem GET navigiert, also richten Sie `logoutURL` auf eine kleine eigene Route, die den POST ausführt.

Wenn der Besucher abgemeldet ist, übergeben Sie `sso` nur mit einer `loginURL`. Das Widget zeigt dann eine Anmeldeaufforderung anstelle eines anonymen Kommentarfelds.

## Things that go wrong

`timestamp` ist Epoch **Millisekunden**, darf nicht in der Zukunft liegen und nicht älter als zwei Tage sein. Generieren Sie es auf dem Server in derselben Anfrage, die den Hash berechnet. Die Erzeugung im Browser ist der klassische Fehler: Der Wert unterscheidet sich von dem, der gehasht wurde, und jeder Kommentar wird abgelehnt.

Setzen Sie niemals `isAdmin` oder `isModerator` vom Identitätsanbieter. Die Anmeldung mit einem Val Town‑Konto sagt nichts darüber aus, wer Ihre Seite moderieren sollte.

Siehe den [SSO guide](/guide-sso.html) für die vollständige Feldliste, gruppenbasierte Threads und Badges.