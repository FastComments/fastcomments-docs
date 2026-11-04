Ako vaš val već zna ko je posetilac, Secure SSO tu identitetu predaje vidžetu tako da nikad ne vidi drugi login. Nema endpointa koje treba napraviti ni čega što se poziva u runtime-u: tri vrednosti izračunate na serverskoj strani i prosleđujete ih u konfiguraciji vidžeta.

Val Town nudi login bez konfiguracije putem `std/oauth`, tako da se posetilac može prijaviti Val Town nalogom koji već ima. Zamenite to bilo čime što vaša aplikacija koristi; FastComments deo se ne menja.

## Izgradite payload na serveru

API tajna potpisuje payload i nikada ne sme dospeti do koda u pregledaču. Instalirajte SDK sa npm-a, koji radi na Val Town Deno runtime-u bez izmena:

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

`getPayload()` vraća `{ userDataJSONBase64, verificationHash, timestamp }`. Te tri vrednosti su sve što dospeva do pregledača. Tajna ih potpisuje i zatim se odbacuje, tako da ništa na stranici ne omogućava čitaocu da lažno predstavi drugog korisnika.

## Prosledite ga vidžetu

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

`oauthMiddleware` dodaje `GET /auth/login`, `GET /auth/callback` i `POST /auth/logout` umesto vas. Imajte na umu da je logout **POST**, dok vidžet navigira na `logoutURL` GET zahtevom, pa usmerite `logoutURL` na malu sopstvenu rutu koja šalje POST.

Kada je posetilac odjavljen, prosledite `sso` samo sa `loginURL`. Vidžet tada prikazuje upit za prijavu umesto anonimnog polja za komentare.

## Stvari koje mogu poći naopako

`timestamp` je epoch **milisekunde**, ne sme biti u budućnosti i ne sme biti stariji od dva dana. Generišite ga na serveru u istom zahtevu u kojem se računa hash. Generisanje u pregledaču je klasična greška: vrednost se razlikuje od one koja je heširana i svaki komentar se odbija.

Nikada ne postavljajte `isAdmin` ili `isModerator` na osnovu identity provajdera. Prijava Val Town nalogom ne govori ništa o tome ko treba da moderira vaš sajt.

Pogledajte [SSO vodič](/guide-customizations-and-configuration.html#sso-secure) za kompletnu listu polja, teme ograničene grupama i bedževe.