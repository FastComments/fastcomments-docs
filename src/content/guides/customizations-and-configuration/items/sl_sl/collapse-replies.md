[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Privzeto se odgovori na vrhnje komentarje prikažejo.

To lahko nastavite tako, da mora uporabnik klikniti "Prikaži odgovore" na vrhnjih komentarjih, da vidi podrejene komentarje.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

To lahko prilagodite brez kode, na strani za prilagajanje pripomočka:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Ta nastavitev ne bo vplivala na število vrhnjih komentarjev, ki se naložijo na začetku. Če imate en vrhnji komentar in 29 podrejenih komentarjev, boste s to nastavitvijo vklopljeno:

- Videli boste vrhnji komentar.
- Pod tem komentarjem boste videli "Prikaži odgovore (29)".

Če želite prikazati vse vrhnje komentarje v kombinaciji s to možnostjo, nastavite [začetno stran na -1](#starting-page).