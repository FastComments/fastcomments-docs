[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, odgovori na komentare najviše razine se prikazuju.

Ovo se može konfigurirati tako da korisnik mora kliknuti „Show Replies” na komentarima najviše razine da bi vidio podkomentare.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Sažmi odgovore na komentare najviše razine'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Opcija za sažimanje odgovora u sučelju prilagodbe widgeta, skriva podkomentare iza veze Prikaži odgovore'; title='Sažmi odgovore' app-screenshot-end]

Ova postavka neće utjecati na broj početno učitanih komentara najviše razine. Ako imate jedan komentar najviše razine i 29 podkomentara, uz ovu postavku ćete:

- Vidjeti komentar najviše razine.
- Vidjeti „Show Replies” (29) ispod tog komentara.

Ako želite prikazati sve komentare najviše razine u kombinaciji s ovom opcijom, postavite [početnu stranicu na -1](#starting-page).