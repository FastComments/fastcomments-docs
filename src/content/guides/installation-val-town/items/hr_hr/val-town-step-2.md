`tenantId: "demo"` je zajednički javni sandbox. Radi bez registracije, zbog čega primjeri koriste njega, ali svi ostali koji probaju FastComments pišu u iste teme i svatko ih može moderirati. Prebacite prije nego što objavite bilo što što vam je važno.

Your tenant ID is on the [API secret page](https://fastcomments.com/auth/my-account/api-secret).

A tenant ID je javan i pripada kodu u pregledniku. API tajna nije, i ništa na ovoj stranici ne treba jednu.

## Read it from an environment variable

Val Town valovi su javni na besplatnoj razini, pa je njihov izvor čitljiv svima. Čuvajte sve osjetljive podatke u varijablama okruženja, čitajte ih pomoću `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Samo računi kreirani na eu.fastcomments.com postavljaju ovo na "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Ovo je važnije nego inače na Val Town iz drugog razloga: **remiksanje val-a kopira ključeve varijabli okruženja, ali ne njihove vrijednosti.** Tajna pohranjena u varijabli okruženja ne prati vaš val u račun drugog korisnika. Tajna zapisana u datoteku to čini.

Povratak na `"demo"` omogućuje da val radi za svakoga tko ga remiksira prije postavljanja vlastitog tenanta.

## EU accounts

Račun, njegovi podaci i ključevi nalaze se u jednoj regiji. Ako je vaš kreiran na `eu.fastcomments.com`, svaka konfiguracija widgeta također treba `region: "eu"`, a skripte se učitavaju s `cdn-eu.fastcomments.com`. U suprotnom ostavite oba nepromijenjena.