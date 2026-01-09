[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Po podrazumevanju se prikazuju odgovori na komentare najvišeg nivoa.

Ovo se može podesiti tako da korisnik mora da klikne "Show Replies" na komentarima najvišeg nivoa da bi video njihove podkomentare.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Ovo podešavanje neće uticati na broj komentara najvišeg nivoa koji se inicijalno učitavaju. Ako imate jedan komentar najvišeg nivoa, i 29 njegovih odgovora, sa ovom opcijom uključenom vi ćete:

- Videćete komentar najvišeg nivoa.
- Videćete Prikaži odgovore (29) ispod ovog komentara.

Ako želite da prikažete sve komentare najvišeg nivoa u kombinaciji sa ovom opcijom, podesite [početnu stranicu na -1](#starting-page).

---