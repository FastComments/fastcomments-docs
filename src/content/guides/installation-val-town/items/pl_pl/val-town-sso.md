Jeśli Twój val już wie, kim jest odwiedzający, Secure SSO przekazuje tę tożsamość widgetowi, dzięki czemu nigdy nie zobaczy on drugiego ekranu logowania. Nie ma endpointów do zbudowania ani niczego do wywołania w czasie działania: obliczasz trzy wartości po stronie serwera i przekazujesz je w konfiguracji widgetu.

Val Town oferuje logowanie bez konfiguracji dzięki `std/oauth`, więc odwiedzający może zalogować się kontem Val Town, które już posiada. Zamień to na to, czego używa Twoja aplikacja; część FastComments się nie zmienia.

## Zbuduj payload na serwerze

API secret podpisuje payload i nigdy nie może trafić do kodu przeglądarki. Zainstaluj SDK z npm, które działa na runtime Deno Val Town bez zmian:

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

`getPayload()` zwraca `{ userDataJSONBase64, verificationHash, timestamp }`. Tylko te trzy wartości trafiają do przeglądarki. Secret podpisuje je, a następnie jest odrzucany, więc nic na stronie nie pozwoli czytelnikowi podrobić innego użytkownika.

## Przekaż go do widgetu

[inline-code-attrs-start title = 'Konfiguracja widgetu z SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` dodaje za Ciebie `GET /auth/login`, `GET /auth/callback` i `POST /auth/logout`. Zauważ, że logout to **POST**, podczas gdy widget przechodzi do `logoutURL` za pomocą GET, więc skieruj `logoutURL` na małą własną trasę, która wysyła ten POST.

Gdy odwiedzający jest wylogowany, przekaż `sso` zawierające tylko `loginURL`. Widget wyświetli wtedy monit o logowanie zamiast anonimowego pola komentarzy.

## Rzeczy, które mogą pójść nie tak

`timestamp` to epoch w **milisekundach**, nie może być w przyszłości i nie może być starszy niż dwa dni. Generuj go na serwerze w tym samym żądaniu, w którym obliczasz hash. Generowanie go w przeglądarce to klasyczny błąd: wartość różni się od tej, która została podpisana, i każdy komentarz jest odrzucany.

Nigdy nie ustawiaj `isAdmin` ani `isModerator` na podstawie dostawcy tożsamości. Zalogowanie się kontem Val Town nie mówi nic o tym, kto powinien moderować Twoją stronę.

Zobacz [przewodnik SSO](/guide-customizations-and-configuration.html#sso-secure) po pełną listę pól, wątki ograniczone grupami oraz odznaki.