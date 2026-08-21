Obstajata dva načina, kako prepovedati uporabnikom komentiranje na vašem spletnem mestu s FastComments.

Prvi je, če že poznate njihov e‑mail, ga lahko vnesete na strani <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">prepovedanih uporabnikov</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Seznam prepovedanih uporabnikov pod Moderiraj komentarje, z naslovi e‑mailov, ki so prepovedani, in gumbom za dodajanje nove prepovedi'; title='Stran prepovedanih uporabnikov' app-screenshot-end]

To stran lahko dostopate prek Moderiraj komentarje -> Prepovedani uporabniki

Ko greste prepovedati uporabnika, lahko izberete vrsto, bodisi Permanent ali Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Obrazec za novo prepoved z vnosnim poljem za e‑mail in izbiro vrste prepovedi Permanent ali Permanent Shadow Ban'; title='Prepoved uporabnika' app-screenshot-end]

Drugi način za prepoved uporabnika je s klikom na gumb za prepoved, ki je postavljen na vsak komentar na strani Moderiranje komentarjev.

Ko kliknete gumb za prepoved, se vam prikažejo nekatere možnosti, kjer lahko določite vrsto in trajanje prepovedi.

### E‑mail aliasi

Ko prepovedujete uporabnika po e‑mailu, FastComments samodejno ignorira `+` alias. Na primer, prepoved `user+alias@gmail.com` bo prav tako prepovedala `user@gmail.com` in katerokoli drugo `+` različico tega naslova, kot je `user+other@gmail.com`.

### Sence prepovedi

Sence prepoved je vrsta prepovedi, ki daje vtis, da je bil uporabnikov komentar ali glas uspešno shranjen, čeprav v resnici ni. To je lahko zaželeno v določenih situacijah.

### Prepoved prek IP naslova

Če najemnik ne želi izključiti te možnosti, FastComments podpira prepoved prek IP tako, da shranjuje zgoščeno različico IP naslova komentatorja.

### Iskanje prepovedanih uporabnikov

Ko vaš seznam preseže eno ali dve strani, ga lahko zožite s pomočjo vrstice za iskanje nad tabelo.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Vrstica za iskanje na strani Prepovedani uporabniki z padajočim seznamom Iskanje po, padajočim seznamom Ujemanje in vnosom Vrednost'; title='Iskanje prepovedanih uporabnikov' app-screenshot-end]

Obstajajo trije nadzorni elementi:

- **Search By** izbere, v katerem polju iskati: Any Field, Email, Name, Banned By ali Banned For Saying. Zadnje štiri ustrezajo stolpcem z enakim imenom v tabeli.
- **Match** izbere, kako primerjati. **Contains** najde vašo vrednost kjerkoli v polju, **Equals** pa se ujema s celotnim poljem.
- **Value** je besedilo, ki ga iščete.

Vsako polje se primerja neodvisno od velikosti črk, zato iskanje `SPAMMER@EXAMPLE.COM` najde prepoved, shranjeno kot `spammer@example.com`.

Nekaj stvari, ki jih je vredno vedeti:

- **Banned For Saying** išče besedilo komentarja, zaradi katerega je bil uporabnik prepovedan. Tako najdete vse, ki so bili prepovedani zaradi določenega izraza.
- **Banned By** išče ime moderatorja, ki je izdal prepoved, kar je uporabno za pregled odločitve drugega moderatorja.
- Wildcard prepovedi so shranjene z `*`, zato iskanje **Contains** za `bademail.com` najde prepoved `*@bademail.com`.
- **Name** se ujema z imenom, prikazanim v stolpcu Name, zato najde uporabnika, tudi če je spremenil ime po tem, ko je bil prepovedan, in tudi če ste ustvarili prepoved z vnosom e‑mail naslova, brez imena v času. Ime, zabeleženo pri prepovedi, se še vedno ujema, zato iskanje po starem ali trenutnem imenu deluje.
- **Any Field** išče po e‑mailu, imenu, moderatorju, ki je izdal prepoved, in besedilu prepovedanega komentarja skupaj.

Vaše iskanje je del URL naslova strani, zato lahko filtriran seznam delite z drugimi moderatorji na enak način, kot delite druge moderacijske povezave. Pomikanje po straneh ohranja iskanje, začetek novega iskanja vas vrne na prvo stran, in **Clear** vrne na celoten seznam.