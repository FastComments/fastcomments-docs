Sproži, ko je komentar samodejno označen kot spam s strani vgrajenega mehanizma za zaznavanje spama FastComments - **ne** s strani moderatorja in ne s strani drugega agenta.

### Kontekst, ki ga prejme agent

- Samodejno označen komentar kot spam.
- Neobvezno: zgodovina niti / uporabnika / kontekst strani, kot je konfigurirano.

### Kdo to sproži

Sistem za obdelavo spama platforme. Glejte [Zaznavanje spama](/guide-moderation.html#spam-detection) v vodniku za moderacijo za več podrobnosti.

### Pogoste uporabe

- **Ponoven pregled moderacije** - mehanizem za zaznavanje spama ima visok priklic, vendar nepopolno natančnost; agent, usposobljen za slog vaše skupnosti, lahko ujame lažno pozitivne. Agent lahko sproži ukaz za odznačitev nepravilno razvrščenega komentarja.
- **Samodejno odblokiranje** - če vaš najemnik agresivno blokira nove račune zaradi spama, lahko agent na tem sprožilcu pregleda in počisti očitne lažno pozitivne primere, preden jih človek sploh vidi.

### Pomembno

- Sprožilec se **ne** sproži za spam, označen s strani moderatorja (uporabite [Sprožilec: Moderator označil spam](#trigger-moderator-spammed)) niti za spam, označen s strani drugega agenta.
- Komentar, ki je bil samodejno označen kot spam in ga moderator kasneje označi kot 'Ni spama', ne sproži sprožilca ponovno.
- Naročanje na ta sprožilec je najbolj uporabno v najemnikih, kjer je način samodejnega označevanja spama mehanizma omogočen v Nastavitvah moderacije. V nasprotnem primeru se sprožilec ne bo sprožil.