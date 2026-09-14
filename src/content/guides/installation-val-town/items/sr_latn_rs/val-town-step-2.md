`tenantId: "demo"` je zajednički javni sandbox. Radi bez registracije, zbog čega se primeri koriste, ali svi ostali koji probaju FastComments pišu u iste teme i svako može da ih moderira. Prebacite pre nego što objavite bilo šta što vam je važno.

Vaš tenant ID je na [API secret page](https://fastcomments.com/auth/my-account/api-secret).

Tenant ID je javan i nalazi se u kodu preglednika. API tajna nije, i ništa na ovoj stranici ne zahteva tajnu.

## Pročitajte ga iz promenljive okruženja

Val Town val‑ovi su javni na besplatnom nivou, pa je njihov izvor čitljiv svima. Čuvajte sve osetljive podatke u promenljivama okruženja, čitajte ih pomoću `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Samo nalozi kreirani na eu.fastcomments.com postavljaju ovo na "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Ovo je značajnije nego obično na Val Town iz drugog razloga: **remiksovanje val‑a kopira ključeve promenljivih okruženja, ali ne i njihove vrednosti.** Tajna čuvana u promenljivoj okruženja ne prati vaš val u nečiji drugi nalog. Tajna zapisana u fajl to čini.

Vraćanje na `"demo"` omogućava da val radi za svakoga ko ga remiksuje pre nego što postavi sopstveni tenant.

## EU nalozi

Nalog, njegovi podaci i ključevi žive u jednoj regiji. Ako je vaš kreiran na `eu.fastcomments.com`, svaka konfiguracija widgeta takođe treba `region: "eu"`, a skripte se učitavaju sa `cdn-eu.fastcomments.com`. U suprotnom ostavite oba nepromenjena.