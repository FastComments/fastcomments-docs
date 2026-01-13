[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, odgovori na komentare najviše razine su vidljivi.

To se može konfigurirati tako da korisnik mora kliknuti "Prikaži odgovore" na komentarima najviše razine da bi vidio njihove podkomentare.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Ova postavka neće utjecati na broj komentara najviše razine koji se inicijalno učitavaju. Ako imate jedan komentar najviše razine i 29 podkomentara, s uključenom ovom postavkom vidjet ćete:

- Vidjet ćete komentar najviše razine.
- Vidjet ćete 'Prikaži odgovore (29)' ispod ovog komentara.

Ako želite prikazati sve komentare najviše razine u kombinaciji s ovom opcijom, postavite [početnu stranicu na -1](#starting-page).

---