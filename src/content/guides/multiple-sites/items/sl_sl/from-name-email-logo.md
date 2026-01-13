Včasih mora FastComments poslati e-pošto vašim uporabnikom, zlasti če ne uporabljate Secure SSO.

Primeri tega vključujejo preverjanje njihovega računa ali aktivnosti ob prvem komentiranju. FastComments jim bo tudi poslal obvestila o odgovorih na njihove komentarje.

Ko FastComments pošilja e-pošto vašim uporabnikom, bomo kot privzeto prikazano ime in e-poštni naslov pošiljatelja uporabili `FastComments Robot` in `noreply@fastcomments.com`.

V nogi teh e-poštnih sporočil bomo prav tako uporabili naš logotip.

Če imate FastComments Flex ali Pro, je to mogoče prilagoditi za vsako domeno posebej na strani "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Pri prilagajanju logotipa, prikazanega v e-pošti, zagotovite, da je velikost, ki jo nalagate, enaka velikosti, ki jo želite prikazati v nogi e-pošte.

### Pri prilagajanju `From Domain`

Če prilagodite `From Domain`, morajo ponudniki e-pošte in odjemalci vedeti, da je FastComments pooblaščen za pošiljanje e-pošte v vašem imenu. V nasprotnem primeru lahko določitev `From Domain` brez izvedbe spodnjih korakov povzroči, da bodo e-poštna sporočila končala v neželeni pošti.

#### 1. Nastavitev SPF

Da omogočite FastComments varno pošiljanje e-pošte v imenu vaše domene, dodajte SPF zapis, ki nam to dovoljuje.

Poskrbite, da bodo SPF zapisi dovoljevali `mail.fastcomments.com` in `sib.fastcomments.com` pošiljanje e-pošte v imenu vaše domene.

Več informacij o tem, kako to narediti, najdete tukaj: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Nastavitev DKIM

Poleg SPF bi morali nastaviti tudi DKIM. Ko je vaša DNS konfiguracija pripravljena, lahko na strani z nastavitvami domene kliknete "Show Advanced", da prikažete DKIM nastavitve za posamezno domeno.

Lahko tudi [invoke the API](/guide-api.html#domain-config-structure) za nastavitev DKIM konfiguracije.

### Povezave za odjavo

Pri uporabi SSO se funkcije za odjavo, uporabljene v e-poštah in obvestilih, lahko prilagodijo [via the DomainConfigs API](/guide-api.html#domain-config-structure).