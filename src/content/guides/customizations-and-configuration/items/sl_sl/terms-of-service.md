FastComments vam omogoča, da od prvih komentatorjev zahtevate sprejem vaših Pogojev uporabe (Terms of Service, TOS) pred oddajo komentarja.

Ko je omogočeno:
- **Anonimni uporabniki** bodo pri vsakem komentarju videli potrditveno polje TOS
- **Prijavljeni uporabniki** bodo potrditveno polje videli le pri svojem prvem komentarju ali kadar posodobite TOS

### Konfiguracija

Pojdite na stran za prilagajanje pripomočka in omogočite potrditveno polje "Require Terms of Service acceptance". Ko je omogočeno, boste videli naslednje možnosti:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: Privzeto potrditveno polje prikaže besedilo "I agree to the Terms of Service and Privacy Policy" s povezavami do obeh dokumentov. Izberite "Customize text per locale", da zagotovite svoje besedilo za posamezen jezik.
- **TOS Last Updated Date**: Ko posodobite svoje Pogoje uporabe, nastavite ta datum. Uporabniki, ki so sprejeli pred tem datumom, bodo morali ponovno sprejeti.

### Kako deluje

- Časovni žig sprejema TOS je shranjen za vsakega uporabnika in za vsak komentar
- Ko uporabnik sprejme TOS, je datum zabeležen v njegovem uporabniškem profilu (na ravni najemnika)
- Če nastavite datum "Last Updated", ki je po datumu sprejema uporabnika, bodo uporabniki morali ponovno sprejeti
- Za anonimne uporabnike, ki jih ni mogoče slediti, se potrditveno polje prikaže pri vsakem oddajanju komentarja