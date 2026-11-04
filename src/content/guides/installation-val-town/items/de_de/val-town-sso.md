Wenn dein Val bereits weiß, wer der Besucher ist, übergibt Secure SSO diese Identität an das Widget, sodass der Nutzer nie eine zweite Anmeldung sieht. Es gibt keine Endpunkte zu bauen und nichts zur Laufzeit aufzurufen: Du berechnest drei Werte serverseitig und übergibst sie in der Widget-Konfiguration.

Val Town bietet Login ohne Konfiguration mit `std/oauth`, sodass sich der Besucher mit dem Val Town-Konto anmelden kann, das er bereits hat. Tausche das gegen alles aus, was deine App verwendet; der FastComments-Teil ändert sich nicht.

## Payload auf dem Server erstellen

Der API-Schlüssel signiert den Payload und darf niemals Browser-Code erreichen. Installiere das SDK von npm, das auf Val Towns Deno-Runtime unverändert funktioniert:

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

`getPayload()` gibt `{ userDataJSONBase64, verificationHash, timestamp }` zurück. Diese drei Werte sind alles, was den Browser erreicht. Der Schlüssel signiert sie und wird dann verworfen, sodass nichts auf der Seite einem Leser ermöglicht, einen anderen Benutzer zu fälschen.

## An das Widget übergeben

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

  // ...render the widget with this config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` fügt `GET /auth/login`, `GET /auth/callback` und `POST /auth/logout` für dich hinzu. Beachte, dass Logout ein **POST** ist, während das Widget mit einem GET zu `logoutURL` navigiert, also richte `logoutURL` auf eine kleine eigene Route, die den POST absendet.

Wenn der Besucher ausgeloggt ist, übergib `sso` nur mit einer `loginURL`. Das Widget zeigt dann eine Login-Aufforderung anstelle eines anonymen Kommentarfelds.

## Häufige Probleme

`timestamp` ist in Epoch-**Millisekunden**, darf nicht in der Zukunft liegen und darf nicht älter als zwei Tage sein. Generiere es auf dem Server in derselben Anfrage, die den Hash berechnet. Die Generierung im Browser ist der klassische Fehler: Der Wert unterscheidet sich vom gehashten Wert und jeder Kommentar wird abgelehnt.

Setze niemals `isAdmin` oder `isModerator` vom Identity-Provider. Die Anmeldung mit einem Val Town-Konto sagt nichts darüber aus, wer deine Seite moderieren sollte.

Siehe den [SSO-Leitfaden](/guide-customizations-and-configuration.html#sso-secure) für die vollständige Feldliste, gruppenbeschränkte Threads und Abzeichen.