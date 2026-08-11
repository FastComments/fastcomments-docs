[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Privzeto se odgovori na komentarje najvišje ravni prikažejo.

To je mogoče nastaviti tako, da mora uporabnik klikniti "Show Replies" na komentarjih najvišje ravni, da vidi podkomentarje.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Strni odgovore na komentarje najvišje ravni'; code-example-end]

To je mogoče prilagoditi brez kode na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Možnost strnitve odgovorov v uporabniškem vmesniku za prilagajanje gradnika, ki skriva podkomentarje za povezavo Show Replies link'; title='Strni odgovore' app-screenshot-end]

Ta nastavitev ne bo vplivala na število začetno naloženih komentarjev najvišje ravni. Če imate en komentar najvišje ravni in 29 podkomentarjev, boste z vklopljeno nastavitvijo:

- Videli boste komentar najvišje ravni.
- Videli boste Show Replies (29) pod tem komentarjem.

Če želite prikazati vse komentarje najvišje ravni v kombinaciji s to možnostjo, nastavite [začetno stran na -1](#starting-page).