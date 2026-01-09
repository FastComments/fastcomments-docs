Po podrazumevanju, korisnici mogu da obrišu svoje komentare. Takođe, brisanje njihovog komentara automatski
briše sve podređene i privremene komentare u niti. Ovo ponašanje važi i uživo.

Možete ovo ograničiti na sledeće načine:

- Umesto toga, anonimizujte obrisani komentar (postavite name i text na `[deleted]` ili na prilagođenu vrednost).
- Ne dozvolite brisanje komentara kada postoje odgovori. Prikazuje se prilagodljiva poruka o grešci.
- Ograničite mogućnost brisanja komentara koji imaju odgovore samo na administratore i moderatore.

Ovo se može konfigurisati putem odeljka `Comment Thread Deletion` u UI za prilagođavanje widgeta.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]