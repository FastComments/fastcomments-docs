Po defaultu, korisnici mogu obrisati svoje komentare. Takođe, brisanje njihovog komentara automatski
briše sve podređene i privremene komentare u niti. Ovo ponašanje je takođe uživo.

You can restrict this in the following ways:

- Umjesto toga, anonimizirajte obrisani komentar (postavite ime i tekst na `[deleted]` ili prilagođenu vrijednost).
- Ne dozvoljavajte brisanje komentara kada postoje odgovori. Prikazuje se prilagodljiva poruka o grešci.
- Ograničite brisanje komentara sa odgovorima samo na administratore i moderatore.

Ovo se može konfigurirati putem odjeljka `Comment Thread Deletion` u Widget Customization UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]