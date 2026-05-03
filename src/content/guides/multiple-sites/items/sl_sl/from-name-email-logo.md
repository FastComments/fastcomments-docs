Včasih mora FastComments poslati e-pošto vašim uporabnikom, zlasti če ne uporabljate Secure SSO.

Primeri tega vključujejo preverjanje njihovega računa ali dejavnosti, ko komentirajo prvič. FastComments jim bo tudi poslal obvestila o odgovorih na njihove komentarje.

Ko FastComments pošilja e-pošto vašim uporabnikom, bo uporabljal privzeto ime pošiljatelja in e-poštni naslov: `FastComments Robot` in `noreply@fastcomments.com`.

V nogi teh e-poštnih sporočil bomo prav tako uporabili naš logotip.

Če imate FastComments Flex ali Pro, je to mogoče prilagoditi za vsako domeno posebej na strani "Moje domene":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Pri prilagajanju logotipa, ki se prikazuje v e-poštnih sporočilih, poskrbite, da je velikost, ki jo nalagate, enaka velikosti, ki jo želite prikazati v nogi e-pošte.

### Ko prilagajate `From Domain`

Če prilagodite `From Domain`, morajo ponudniki in odjemalci e-pošte vedeti, da je FastComments pooblaščen za pošiljanje e-pošte v vašem imenu. V nasprotnem primeru bo določitev `From Domain` brez upoštevanja spodnjih korakov verjetno povzročila, da bodo e-poštna sporočila končala v mapi z neželeno pošto.

#### 1. Nastavitev SPF

Da omogočite FastComments varno pošiljanje e-pošte v imenu vaše domene, dodajte zapis SPF, ki nam to dovoljuje.

Poskrbite, da obstajajo SPF zapisi, ki dovoljujejo `mail.fastcomments.com` in `sib.fastcomments.com` pošiljanje pošte v imenu vaše domene.

Več informacij o tem, kako to narediti, najdete tukaj: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Nastavitev DKIM

Poleg SPF bi morali nastaviti tudi DKIM. Ko bo vaša DNS konfiguracija pripravljena, lahko na strani konfiguracije domene kliknete "Prikaži napredno", da prikažete DKIM nastavitve za posamezno domeno.

Prav tako lahko [pokličete API](/guide-api.html#domain-config-structure) za nastavitev DKIM konfiguracije.

### Povezave za odjavo

Pri uporabi SSO se lahko funkcije za odjavo, uporabljene v e-pošti in obvestilih, prilagodijo [prek DomainConfigs API](/guide-api.html#domain-config-structure).

### Zameglitev povezav v e-pošti

Če ugled vaše domene povzroča, da obvestilna e-pošta konča v mapi z neželeno pošto, lahko preusmerite gumbe "Prikaži komentar" prek `fastcomments.com` namesto, da bi povezovali neposredno na vašo stran. Ponudniki e-pošte ocenjujejo vsako povezavo v telesu sporočila glede na ugled ciljne strani, zato, kadar je vaša domena označena, same goli povezavi prispevajo k rezultatu neželene pošte ne glede na to, kako urejena je vaša konfiguracija pošiljanja.

Omogočite to v "Prikaži napredno" na strani Moje domene, v razdelku "Zameglitev povezav v e-pošti". Nastavitev je na ravni domene.

Ko je omogočeno, se povezave v e-poštnih sporočilih tipov mention, reply, new-comment, subscribed-page, profile-comment, and digest prepišejo v kratke žetone, ki ob kliku preusmerijo na izvirno stran. Cilj je vezan na vaš tenant: preusmeritev naprej pošlje le na URL-je, katerih gostitelj se ujema z eno izmed vaših konfiguriranih domen, in žetoni se samodejno potečejo po 30 dneh.

Izkušnja ob preusmeritvi se ne spremeni. Bralci še vedno pristanejo na vaši strani z avtomatsko pomaknjenim komentarjem v pogled.