Flex račun je seznam postavk, ena za vsako vrsto uporabe, pri čemer vsaka prikazuje količino in ceno, na primer `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Cene so na [strani s cenami](https://fastcomments.com/traffic-pricing) in na vaši strani [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Ta razdelek pojasnjuje, kaj pomeni vsaka količina.

### Šteje se skozi mesec

Te se vsak mesec ponastavijo in se štejejo od prvega do zadnjega dne koledarskega meseca (UTC):

- **Page Loads**: število nalaganj gradnika za komentarje ali vira na vaših straneh. Vsako nalaganje gradnika se šteje, ne glede na to, koliko komentarjev prikaže. Ni zaračunavanja na nit komentarjev ali na stran.
- **API Credits**: klici na REST API z vašim API ključem.
- **Small Widgets Credits**: nalaganja malih gradnikov, kot so števci komentarjev in značke.
- **LLM Tokens**: žetoni, ki jih uporabljajo [AI Agents](/guide-ai-agents.html) in druge funkcije LLM.

### Šteje se na koncu meseca

To je posnetek stanja, narejen ob ustvarjanju računa:

- **Comments Stored**: skupno število komentarjev v vašem računu, ne le tistih, dodanih ta mesec. Brisanje komentarjev to zmanjša, le da izbrisan komentar z odgovori ostane kot vsebnik, da odgovori še vedno smiselni.
- **SSO Users**: skupno število SSO uporabnikov, ki jih je ustvarila vaša stran, vključno z ljudmi, ki so komentirali samo enkrat pred dolgim časom. Če je to največji del vašega računa, vprašajte podporo o zaračunavanju po mesečno aktivnih uporabnikih, kjer se štejejo samo uporabniki, ki so se v mesecu prijavili.
- **Moderators**, **Admins**, **SSO Moderators** in **SSO Admins**: sedeži, obravnavani v razdelku [Admin and Moderator Seats](#admin-and-moderator-seats). Prvi admin je brezplačen.
- **Domains**: strani v vašem računu. Prva stran je brezplačna, vsaka dodatna pa se zaračuna mesečno, dokler je v računu. Poddomena se šteje le, če jo dodate kot svojo stran; ena vrhnja domena pokriva svoje poddomene. Glejte [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: podračuni, ki jih upravlja ta račun, če obstajajo, plus njihova lastna uporaba kot ločene postavke.

### Minimum

Če postavke skupaj znašajo manj od Flex minimuma, se račun dopolni do minimuma. Majhna stran z enim adminom in eno domeno plača samo minimum.

### Predogled računa

Stran [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) prikazuje vsako postavko naslednjega računa z današnjimi številkami, tako da ni presenečenj. Števci nalaganj strani, API in malih gradnikov so lahko zaostali do eno minuto; ostalo je v živo.

### Druge valute

Flex uporaba je ocenjena v ameriških dolarjih. Če je valuta vašega računa drugačna, se vsaka postavka pretvori po tečaju na dan ustvarjanja računa, zato lahko ista uporaba od meseca do meseca stane nekoliko drugače. Glejte [Currencies](#currencies).