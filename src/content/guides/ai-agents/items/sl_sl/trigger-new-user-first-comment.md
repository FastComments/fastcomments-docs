Sproži se, ko uporabnik objavi svoj prvi komentar na tem spletnem mestu (vaš tenant). To se zgodi **enkrat na uporabnika** - nadaljnji komentarji istega uporabnika ga ne sprožijo ponovno.

### Kontekst, ki ga prejme agent

- Nov komentar.
- Neobvezni kontekst nit/zgodovine uporabnika/strani glede na nastavitve.

Ko je kontekst uporabnikove zgodovine vklopljen, bo seveda seznam uporabnikovih zadnjih komentarjev prazen (ali bo vseboval samo tega), vendar sta faktor zaupanja in starost računa določena.

### Pomembno

- "Prvi komentar na tem spletnem mestu" je omejen na **tenant**, ne na vse strani v FastComments. Uporabnik, ki ima komentarje na drugih straneh FastComments, bo vseeno sprožil ta sprožilec, ko prvič objavi pri vas.
- Sprožilec se sproži le za uporabnike z userId. Anonimni nepotrjeni komentarji brez stabilnega userId ga ne sprožijo.
- Sprožilec se sproži, ko je komentar odobren/viden (ne ob prvotnem objavljanju). Urejanja ali ukrepi moderatorja pri prvih komentarjih ga ne sprožijo znova.

### Pogoste uporabe

- **Pozdrav dobrodošlice** - predloga [Welcome Greeter template](#template-welcome-greeter) je zasnovana okoli tega sprožilca.
- **Uvajanje** - pošljite [DM warning](#tool-warn-user) (tukaj uporabljeno kot prijazen opomnik in ne kot opozorilo), ki uporabnika usmeri na pravila skupnosti.
- **Obvestilo pregledovalcu** - če želite, da človek pregleda prvi prispevek vsakega novega komentatorja, lahko [`mark_comment_reviewed`](#tools-overview) označi komentar za pregled.