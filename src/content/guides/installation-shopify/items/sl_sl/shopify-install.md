### Namestitev iz trgovine Shopify App Store

1. Odprite [FastComments listing on the Shopify App Store](https://apps.shopify.com/fastcomments).
2. Kliknite **Add app** in izberite načrt, ki ga želite med postopkom namestitve.
3. Ko je namestitev končana, vas Shopify preusmeri nazaj v upravljanje FastComments znotraj Shopify.

To je celotna namestitev. Ničesar ni treba prilepiti v datoteke teme.

### Kaj se nastavi za vas

Namestitev izvede vse, kar bi sicer morali narediti ročno:

- Za vašo trgovino se ustvari FastComments tenant in je povezan z domeno vaše trgovine.
- URL vaše trgovine se doda v pooblastila domen tenanta, tako da se komentarji naložijo brez napake zaradi domene.
- Zapiše se shop metafield `fastcomments.tenant_id`, da vsak blok ve, proti kateremu tenantu naj se renderira.
- Enotna prijava (single sign-on) za vaše stranke Shopify je privzeto omogočena.
- Zaračunavanje poteka prek Shopify Managed Pricing. Zaračunavanja se pojavijo na vaši običajni Shopify izstavitvi. Nadgradite, znižajte načrt ali prekličite v **Settings > Apps and sales channels > FastComments** v vašem Shopify upravljanju.

Če je bila vaša trgovina že pred namestitvijo aplikacije stranka FastComments, namestitev ponovno uporabi obstoječega tenanta namesto ustvarjanja novega.

### Vgrajen skrbniški vmesnik

Ko odprete aplikacijo FastComments iz svojega Shopify upravljanja, pristaneš na nadzorni plošči z enim klikom do celotnega FastComments backend-a:

- **Nadzorna plošča**: nastavitve računa, uporaba in podrobnosti naročnine.
- **Vrsta za moderiranje**: odobritev, zavrnitev in odgovarjanje na komentarje po celotni trgovini.
- **Prilagodi**: prilagodite barve pripomočka, pisave, pravila moderiranja in konfiguracijo.
- **Pomočnik za ocene in mnenja**: nastavite ocene z zvezdicami in vprašanja za ocene, če želite uporabiti blok Reviews Summary.

Vsaka ploščica odpre FastComments z enkratno povezavo za prijavo, zato ločena prijava ni potrebna.

### Naslednje: dodajte bloke v vašo trgovino

Odprite urejevalnik teme Shopify (**Spletna trgovina > Teme > Prilagodi**), odprite predlogo, v katero želite dodati komentarje ali ocene, in kliknite **Add block**. FastComments bloki se prikažejo pod **Apps**. Preostali del tega vodiča obravnava vsak od njih.