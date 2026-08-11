[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Podrazumevano, odgovori na komentare najvišeg nivoa se prikazuju.

Ovo se može konfigurisati tako da korisnik mora da klikne „Show Replies“ na komentarima najvišeg nivoa da bi video podkomentare.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Opcija za sakrivanje odgovora u UI za prilagođavanje widgeta, sakriva podkomentare iza linka Show Replies'; title='Sakrij odgovore' app-screenshot-end]

Ovo podešavanje neće uticati na broj početno učitanih komentara najvišeg nivoa. Ako imate jedan komentar najvišeg nivoa i 29 podkomentara, uz ovo podešavanje ćete:

- Videti komentar najvišeg nivoa.
- Videti „Show Replies (29)“ ispod tog komentara.

Ako želite da prikažete sve komentare najvišeg nivoa u kombinaciji sa ovom opcijom, postavite [početnu stranicu na -1](#starting-page).