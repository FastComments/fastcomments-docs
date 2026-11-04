Ako vaš val već zna tko je posjetitelj, Secure SSO predaje taj identitet widgetu tako da posjetitelj nikada ne vidi drugu prijavu. Nema endpointa koje treba izgraditi ni ničega što treba pozvati u runtimeu: tri vrijednosti izračunate su na strani servera i prosljeđuju se u konfiguraciji widgeta.

Val Town nudi prijavu bez konfiguracije putem `std/oauth`, tako da se posjetitelj može prijaviti Val Town računom koji već ima. Zamijenite to onim što koristi vaša aplikacija; FastComments dio se ne mijenja.

## Izgradnja payloada na serveru

API tajna potpisuje payload i nikada ne smije dospjeti u kod preglednika. Instalirajte SDK s npm-a, koji radi na Val Town Deno runtimeu bez izmjena:

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

`getPayload()` vraća `{ userDataJSONBase64, verificationHash, timestamp }`. Te tri vrijednosti su sve što dospijeva u preglednik. Tajna ih potpisuje i zatim se odbacuje, tako da ništa na stranici ne omogućuje čitatelju da krivotvori drugog korisnika.

## Prosljeđivanje widgetu

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

`oauthMiddleware` dodaje `GET /auth/login`, `GET /auth/callback` i `POST /auth/logout` umjesto vas. Imajte na umu da je odjava **POST**, dok widget navigira na `logoutURL` GET zahtjevom, pa usmjerite `logoutURL` na malu vlastitu rutu koja šalje POST zahtjev.

Kada je posjetitelj odjavljen, proslijedite `sso` samo s `loginURL`. Widget tada prikazuje upit za prijavu umjesto anonimnog okvira za komentare.

## Stvari koje mogu poći po krivom

`timestamp` je u epoch **milisekundama**, ne smije biti u budućnosti i ne smije biti stariji od dva dana. Generirajte ga na serveru u istom zahtjevu u kojem se računa hash. Generiranje u pregledniku klasična je greška: vrijednost se razlikuje od one koja je potpisana i svaki komentar se odbacuje.

Nikada ne postavljajte `isAdmin` ili `isModerator` na temelju davatelja identiteta. Prijava Val Town računom ne govori ništa o tome tko treba moderirati vašu stranicu.

Pogledajte [SSO vodič](/guide-customizations-and-configuration.html#sso-secure) za potpuni popis polja, threadove ograničene grupama i bedževe.