Ko uporabniki komentirajo ali glasujejo, in niso prijavljeni, bodo pozvani, da vneseta svoj e-poštni naslov in uporabniško ime.

Za nekatere spletna mesta je zahtevanje, da obiskovalec izmisli edinstveno uporabniško ime, ovira, zlasti na mobilnih napravah. FastComments lahko
ustvari nevtralno uporabniško ime za vsakega novega obiskovalca in ga vnaprej izpolni v polje za uporabniško ime, na primer `BraveOtter4172`.

Obiskovalec ga lahko pusti takšnega, kot je, ali ga zamenja z imenom po svoji izbiri.

To je mogoče omogočiti v vmesniku za prilagajanje, pod nastavitvijo z imenom `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Možnost Generate Usernames Automatically v vmesniku za prilagajanje pripomočka'; title='Samodejno generiranje uporabniških imen' app-screenshot-end]

#### Kako deluje

- Vsako generirano ime je edinstveno. Preveri se glede na obstoječe račune in je rezervirano za brskalno sejo tistega obiskovalca, tako da dva obiskovalca ne dobita istega imena.
- Ime se generira samo za obiskovalce, ki ga še nimajo. Prijavljeni uporabniki, uporabniki SSO in obiskovalci, ki so že komentirali, obdržijo svoje obstoječe ime.
- Deluje z anonimnim komentiranjem ali brez njega ([anonymous commenting](/guide-customizations-and-configuration.html#allow-anon)). Ko je anonimno komentiranje izklopljeno, obiskovalec še vedno vpiše svoj e-poštni naslov, vendar si ne mora več izmisliti uporabniškega imena.
- Vrnjeni obiskovalec, ki vpiše e-poštni naslov, ki ga je že uporabljal, se poveže z obstoječim računom in obdrži ime na tem računu.
- Če je nastavljeno tudi [Privzeto uporabniško ime](/guide-customizations-and-configuration.html#default-username), ima generirano ime prednost.