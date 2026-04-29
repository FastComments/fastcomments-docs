Sproži se, ko moderator označi komentar kot spam.

### Kontekst, ki ga prejme agent

- Komentar, z zastavico po dejanju `Is Spam`.
- **ID uporabnika, ki je sprožil dogodek** - moderator, ki je ukrepal.
- Neobvezna zgodovina niti / uporabnika / kontekst strani, kot je konfigurirano.

### Kdo to sproži

Človeško moderatorsko dejanje. Spam oznake, ki jih ustvari agent (prek [`mark_comment_spam`](#tools-overview)) **ne** sprožijo tega sprožilca.

### Pogoste uporabe

- **Zabeležitev spomina** - naj agent shrani opombo v spomin o spamanem uporabniku (npr. "prej označen kot spam za X s strani moderatorja"), da bodo prihodnji moderatorski agenti imeli kontekst.
- **Uveljavljanje na ravni uporabnika** - moderator, ki označi komentar kot spam, je lahko signal agentu, da izda opozorilo ali kratkotrajno prepoved, pod pogojem odobritve.
- **Zrcaljenje v zunanji sistem** preko [Webhooks](#webhooks-overview).

---