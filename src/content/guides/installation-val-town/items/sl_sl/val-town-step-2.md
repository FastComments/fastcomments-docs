`tenantId: "demo"` je deljeni javni peskovnik. Deluje brez registracije, zato ga primeri uporabljajo, vendar vsi drugi, ki preizkušajo FastComments, pišejo v iste niti in kdorkoli jih lahko moderira. Zamenjajte ga, preden objavite karkoli, kar vam je pomembno.

Vaš tenant ID je na [API secret page](https://fastcomments.com/auth/my-account/api-secret).

Tenant ID je javni in naj bi bil v kodi brskalnika. API skrivnost ne sme biti, in nič na tej strani tega ne zahteva.

## Read it from an environment variable

Val Town vrednosti so javne v brezplačnem sloju, zato je njihov vir svetovno berljiv. Vse občutljive podatke shranite v spremenljivke okolja, preberite jih z `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Le računi, ustvarjeni na eu.fastcomments.com, nastavi to na "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

To je pomembnejše kot običajno na Val Town iz drugega razloga: **ponovno mešanje vrednosti (remixing) kopira ključe spremenljivk okolja, ne pa njihove vrednosti.** Skrivnost, shranjena v spremenljivki okolja, ne sledi vašemu valu v račun drugega uporabnika. Skrivnost, zapisana v datoteko, sledi.

Vrnitev na `"demo"` ohranja val delujoč za vsakogar, ki ga ponovno meša, preden nastavi svojega najemnika.

## EU accounts

Račun, njegovi podatki in ključi živijo v eni regiji. Če je vaš ustvarjen na `eu.fastcomments.com`, mora vsaka konfiguracija gradnika prav tako vsebovati `region: "eu"`, skripte pa se nalagajo iz `cdn-eu.fastcomments.com`. V nasprotnem primeru pustite oba nespremenjena.