`tenantId: "demo"` je javni dijeljeni sandbox. Radi bez registracije, zbog čega ga primjeri koriste, ali svi ostali koji isprobavaju FastComments pišu u iste niti i svatko ih može moderirati. Prebacite se prije nego što objavite bilo što vam važno.

Vaš tenant ID nalazi se na [API secret stranici](https://fastcomments.com/auth/my-account/api-secret).

Tenant ID je javan i pripada u kod u pregledniku. API secret nije, i ništa na ovoj stranici ne treba jedan.

## Pročitajte ga iz varijable okruženja

Val Town valovi su javni na besplatnom planu, pa je njihov izvor čitljiv svima. Sve osjetljivo držite u varijablama okruženja, čitajte s `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Samo računi stvoreni na eu.fastcomments.com ovo postavljaju, na "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

To je na Val Townu važnije nego obično iz još jednog razloga: **remixanje vala kopira ključeve varijabli okruženja, ali ne i njihove vrijednosti.** Tajna u varijabli okruženja ne prati vaš val u tuđi račun. Tajna zapisana u datoteku prati.

Povratak na `"demo"` održava val funkcionalnim za svakoga tko ga remixira prije postavljanja vlastitog tenanta.

## EU računi

Račun, njegovi podaci i njegovi ključevi žive u jednoj regiji. Ako je vaš stvoren na `eu.fastcomments.com`, svaka konfiguracija widgeta također treba `region: "eu"`, a skripte se učitavaju s `cdn-eu.fastcomments.com`. Inače ostavite oboje na miru.