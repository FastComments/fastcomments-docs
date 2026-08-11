---
Včasih FastComments mora poslati e‑pošto vašim uporabnikom, še posebej, če ne uporabljate varnega SSO.

Primeri tega vključujejo preverjanje njihovega računa ali dejavnosti, ko komentirajo prvič. FastComments jim bo prav tako poslal obvestila o odgovorih na njihove komentarje.

Ko FastComments pošilja e‑pošto vašim uporabnikom, bomo uporabili privzeto ime pošiljatelja in e‑pošto `FastComments Robot` ter `noreply@fastcomments.com`.

V podnožju teh e‑pošt bomo prav tako uporabili naš lastni logotip.

Če imate FastComments Flex ali Pro, je vse to mogoče prilagoditi na podlagi domene prek strani "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Obrazec za nastavitve e‑pošte po domenah z polji za ime pošiljatelja, e‑pošto pošiljatelja in nalaganje logotipa'; title='Prilagajanje imena pošiljatelja, e‑pošte in logotipa' app-screenshot-end]

Ko prilagajate logotip, prikazan v e‑poštah, zagotovite, da je velikost, ki jo nalagate, enaka velikosti, ki jo želite prikazati v podnožju e‑pošte.

### Pri prilagajanju `From Domain`

Če prilagodite `From Domain`, morajo ponudniki e‑pošte in odjemalci vedeti, da je FastComments pooblaščen za pošiljanje e‑pošte v vašem imenu. V nasprotnem primeru bo določitev `From Domain` brez upoštevanja spodnjih korakov verjetno povzročila, da e‑pošta konča v neželeni pošti.

#### 1. Nastavitev SPF

Da FastComments omogočimo varno pošiljanje e‑pošte v vašem imenu, zagotovite, da dodate SPF zapis, ki to omogoča.

Poskrbite, da obstajajo SPF zapisi, ki omogočajo `mail.fastcomments.com` in `sib.fastcomments.com` pošiljanje pošte v vašem imenu.

Več informacij o tem, kako to storiti, je na tem naslovu: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Nastavitev DKIM

Poleg SPF bi morali nastaviti DKIM. Ko je vaša konfiguracija DNS pripravljena, lahko kliknete "Show Advanced" na strani nastavitev domene, da prikažete DKIM nastavitve po domenah.

Lahko tudi [pokličete API](/guide-api.html#domain-config-structure), da nastavite DKIM konfiguracijo.

### Povezave za odjavo

Pri uporabi SSO je mogoče funkcije odjave, uporabljene v e‑poštah in obvestilih, prilagoditi [prek DomainConfigs API](/guide-api.html#domain-config-structure).

### Zameglitev povezav v e‑pošti

Če ugled domene vašega spletnega mesta povzroča, da obvestilna e‑pošta pristane v neželeni pošti, lahko gumbe "view comment" preusmerite prek `fastcomments.com` namesto, da bi jih povezali neposredno na vašo stran. Ponudniki poštnih predalov ocenjujejo vsako povezavo v telesu e‑pošte glede na ugled cilja, zato ko je vaša domena označena, surove povezave prispevajo k oceni neželene pošte ne glede na to, kako čista je vaša pošiljalna nastavitev.

To omogočite pod "Show Advanced" na strani My Domains, v razdelku "Email Link Obfuscation". Nastavitev je po domeni.

Ko je omogočeno, se povezave v omenah, odgovorih, novih komentarjih, naročenih straneh, komentarjih profila in povzetkih e‑pošte prepišejo v kratke žetone, ki preusmerijo na izvirno stran ob kliku. Cilj je vezan na vaš najemnik: preusmeritev pošilja le na URL‑je, katerih gostitelj se ujema z enim od vaših konfiguriranih domen, žetoni pa samodejno potečejo po 30 dneh.

Izkušnja ob kliku ostane nespremenjena. Bralci še vedno pristanejo na vaši strani z komentarjem, ki je že pomaknjen v pogled.