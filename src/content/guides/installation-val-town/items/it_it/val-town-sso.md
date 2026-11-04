Se il tuo val già conosce l'identità del visitatore, Secure SSO passa quell'identità al widget, così non vedrà mai un secondo login. Non ci sono endpoint da costruire e nulla da chiamare a runtime: calcoli tre valori lato server e li passi nella configurazione del widget.

Val Town offre login zero-config con `std/oauth`, quindi il visitatore può accedere con l'account Val Town che già possiede. Sostituiscilo con quello che usa la tua app; la parte FastComments non cambia.

## Costruisci il payload sul server

L'API secret firma il payload e non deve mai finire nel codice del browser. Installa l'SDK da npm, che funziona sul runtime Deno di Val Town così com'è:

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

`getPayload()` restituisce `{ userDataJSONBase64, verificationHash, timestamp }`. Quei tre valori sono tutto ciò che arriva al browser. Il secret li firma e viene poi scartato, quindi nulla nella pagina permette a un lettore di falsificare un utente diverso.

## Passalo al widget

[inline-code-attrs-start title = 'Configurazione del widget con SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` aggiunge per te `GET /auth/login`, `GET /auth/callback` e `POST /auth/logout`. Nota che il logout è una **POST**, mentre il widget naviga verso `logoutURL` con una GET, quindi punta `logoutURL` a una piccola route tua che invia la POST.

Quando il visitatore non è autenticato, passa `sso` con solo un `loginURL`. Il widget mostrerà quindi un prompt di login invece della casella dei commenti anonima.

## Cose che vanno storte

`timestamp` è in **millisecondi** epoch, non deve essere nel futuro e non deve essere più vecchio di due giorni. Genera lato server nella stessa richiesta che calcola l'hash. Generarlo nel browser è l'errore classico: il valore differisce da quello firmato e ogni commento viene rifiutato.

Non impostare mai `isAdmin` o `isModerator` dall'identity provider. Accedere con un account Val Town non dice nulla su chi dovrebbe moderare il tuo sito.

Consulta la [guida SSO](/guide-customizations-and-configuration.html#sso-secure) per l'elenco completo dei campi, i thread con accesso a gruppi e i badge.