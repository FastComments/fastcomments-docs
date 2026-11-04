Če vaš val že ve, kdo je obiskovalec, Secure SSO ta identiteto prenese na pripomoček, tako da obiskovalec nikoli ne vidi druge prijave. Ni vtičnic za izdelavo in ničesar ni treba klicati med izvajanjem: tri vrednosti izračunate na strežniški strani in jih podate v konfiguracijo pripomočka.

Val Town ponuja prijavo brez konfiguracije prek `std/oauth`, tako da se obiskovalec lahko prijavi z računom Val Town, ki ga že ima. To zamenjajte s tem, kar uporablja vaša aplikacija; del FastComments se ne spremeni.

## Izdelava podatkovne vsebine na strežniku

API skrivnost podpiše podatkovno vsebino in nikoli ne sme priti do kode v brskalniku. Namestite SDK iz npm, ki deluje na Val Town's Deno runtime tak, kot je:

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

`getPayload()` vrne `{ userDataJSONBase64, verificationHash, timestamp }`. Te tri vrednosti so vse, kar pride do brskalnika. Skrivnost jih podpiše in se nato zavrže, tako da nič na strani ne omogoča bralcu, da bi ponarejil drugega uporabnika.

## Posredovanje pripomočku

[inline-code-attrs-start title = 'Konfiguracija pripomočka s SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` doda `GET /auth/login`, `GET /auth/callback` in `POST /auth/logout` namesto vas. Upoštevajte, da je odjava **POST**, medtem ko se pripomoček navigira na `logoutURL` z GET, zato usmerite `logoutURL` na majhno lastno pot, ki pošlje POST.

Ko je obiskovalec odjavljen, posredujte `sso` samo z `loginURL`. Pripomoček nato namesto anonimnega okvirja za komentarje prikaže poziv za prijavo.

## Stvari, ki gredo narobe

`timestamp` je epoha v **milisekundah**, ne sme biti v prihodnosti in ne sme biti starejši od dveh dni. Ustvarite ga na strežniku v isti zahtevi, ki izračuna zgoščevalni izvleček. Ustvarjanje v brskalniku je klasična napaka: vrednost se razlikuje od tiste, ki je bila zgoščena, in vsak komentar je zavrnjen.

Nikoli ne nastavite `isAdmin` ali `isModerator` iz ponudnika identitete. Prijava z računom Val Town ne pove nič o tem, kdo naj moderatorira vaše spletno mesto.

Za celoten seznam polj, niti, omejene na skupine, in značke si oglejte [vodnik SSO](/guide-customizations-and-configuration.html#sso-secure).