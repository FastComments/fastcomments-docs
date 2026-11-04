`tenantId: "demo"` je javni peskovnik v skupni rabi. Deluje brez registracije, zato ga uporabljajo primeri, a vsi ostali, ki preizkušajo FastComments, pišejo v iste niti in jih lahko kdorkoli moderira. Preklopite, preden objavite karkoli pomembnega.

Vaš ID najemnika (tenant ID) je na [strani API skrivnosti](https://fastcomments.com/auth/my-account/api-secret).

ID najemnika je javen in spada v kodo brskalnika. API skrivnost pa ne, in nič na tej strani je ne potrebuje.

## Preberite ga iz okoljske spremenljivke

Val Town vali so na brezplačnem paketu javni, zato je njihova izvorna koda berljiva za vse. Vse občutljivo hranite v okoljskih spremenljivkah in berite z `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// To nastavijo samo računi, ustvarjeni na eu.fastcomments.com, in sicer na "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

To je na Val Town pomembnejše kot običajno še iz enega razloga: **remixanje vala kopira ključe okoljskih spremenljivk, ne pa njihovih vrednosti.** Skrivnost, shranjena v okoljski spremenljivki, ne sledi vašemu valu v račun nekoga drugega. Skrivnost, zapisana v datoteko, pa.

Zasilna vrednost `"demo"` ohranja val delujočega za vsakogar, ki ga remixa, preden nastavi svoj ID najemnika.

## EU računi

Račun, njegovi podatki in njegovi ključi živijo v eni regiji. Če je bil vaš račun ustvarjen na `eu.fastcomments.com`, potrebuje vsaka konfiguracija pripomočka tudi `region: "eu"`, skripti pa se nalagajo iz `cdn-eu.fastcomments.com`. Sicer oboje pustite pri miru.