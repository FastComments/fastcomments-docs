Vtičnik podpira tri načine SSO za integracijo uporabniških računov Moodle s FastComments.

#### Varen SSO (Priporočeno)

Podatki o uporabnikih so podpisani na strežniški strani z HMAC-SHA256 z vašim **API Secret**. To je najbolj varna možnost in je priporočena za uporabo v produkciji.

Z Varnim SSO:

- Imena uporabnikov, e-poštni naslovi in avatarji se varno predajo FastComments.
- Skrbniki strani Moodle so samodejno sinhronizirani kot moderatorji FastComments.
- Uporabniki se ne morejo predstavljati kot drugi računi.
- Zahteva, da je v nastavitvah vtičnika konfiguriran **API Secret**.

#### Preprost SSO

Podatki o uporabnikih (ime, e-pošta, avatar) se pošljejo na odjemalčevi strani brez kriptografskega podpisa. To je lažje za nastavitev, ker ne zahteva **API Secret**, vendar je manj varno, ker podatki o uporabnikih niso preverjeni na strežniški strani.

#### Brez

Brez SSO integracije. Uporabniki komentirajo anonimno ali se morajo ločeno prijaviti v FastComments. Uporabite to, če ne želite, da bi bili uporabniški računi Moodle povezani s komentarji.