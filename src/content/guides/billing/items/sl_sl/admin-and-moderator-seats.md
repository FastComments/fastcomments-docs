Pri paketu Flex se osebe, ki lahko upravljajo ali moderirajo vaš račun, obračunavajo kot mesečne sedeže. To je del računa Flex, ki ljudi najpogosteje preseneti, običajno zato, ker so bili sedeži ustvarjeni, ne da bi kdo to opazil.

### Kaj šteje kot sedež

- **Skrbniki (Admins)**: uporabniki na strani `Users` vašega računa s katero koli skrbniško dovoljenjem (obračunavanje, prilagajanje, analitika, moderiranje itd.). Prvi skrbnik, običajno lastnik računa, je brezplačen; vsak dodatni skrbnik je sedež.
- **Moderatorji**: osebe na strani `Moderators`, ki so sprejele svoje vabilo. Čakajoče vabilo ni obračunano. Uporabnik, katerega edino dovoljenje je moderiranje komentarjev, se obračuna kot moderator, ne kot skrbnik.
- **SSO skrbniki** in **SSO moderatorji**: SSO uporabniki, s katerimi se vaše spletno mesto prijavi z `isAccountOwner`, `isAdminAdmin` ali `isCommentModeratorAdmin`, nastavljenimi v SSO payloadu. Obračunavajo se po istih cenah kot običajni skrbniki in moderatorji. Glejte [SSO User Structure](/guide-api.html#sso-user-structure).

Sedeži so deduplikirani po e-poštnem naslovu, tako da se oseba, ki je hkrati skrbnik in moderator, ali ki ima SSO uporabnika in prijavo na nadzorno ploščo z istim e-poštnim naslovom, obračuna enkrat, po višji ceni. Oseba z dvema različnima e-poštnima naslovoma sta dva sedeža.

Pri računih, obračunanih po mesečno aktivnih uporabnikih, SSO moderator, ki se v mesecu ni prijavil, za tisti mesec ni obračunan.

### Sedeži, ki jih niste dodali

Dve funkciji samodejno ustvarjata sedeže:

- **SSO sinhronizacija skrbnikov.** Ko se SSO uporabnik prijavi z e-poštnim naslovom, ki se ujema z enim od uporabnikov vaše nadzorne plošče, se skrbniška ali moderatorska dovoljenja tega uporabnika kopirajo na SSO uporabnika. Pri WordPress SSO to pomeni, da lahko vsak WordPress skrbnik postane FastComments skrbnik ali moderator. To izklopite pod `Customize` z možnostjo `Disable Automatic Admin Migration` na pravilu prilagajanja.
- **Različni e-poštni naslovi za isto osebo.** Če vaša prijava na nadzorno ploščo uporablja en e-poštni naslov, vaš SSO uporabnik pa drugega, sta to dva sedeža. Uporaba istega e-poštnega naslova na obeh jih združi.

### Preverjanje in odstranjevanje sedežev

Stran [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) prikazuje števila vaših `Moderators`, `SSO Admins` in `SSO Moderators`, predogled naslednjega računa pa prikazuje, koliko stanejo. Osebe, ki jih ne potrebujete, odstranite s strani `Users` in `Moderators`, ali pa odstranite skrbniške zastavice iz svojega SSO payloada. Sedeži se štejejo, ko se račun ustvari ob koncu meseca, tako da sedež, odstranjen pred tem, za tisti mesec ni obračunan.

Glejte [Adding Moderators](/guide-users.html#adding-moderators) in [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) za upravljanje obojega.