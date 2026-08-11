---
[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Privzeto bo FastComments prikazal možnosti glasovanja kot puščici za glasovanje gor in dol, kar uporabnikom omogoča, da komentar glasujejo gor ali dol.

Vendar je mogoče spremeniti stil orodne vrstice za glasovanje. Trenutne možnosti so privzeti gumbi Gor/Dol ali uporaba mehanizma glasovanja v slogu srca.

Za to uporabljamo zastavico **voteStyle** na naslednji način:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Omogoči srčni gumb'; code-example-end]

Močno priporočamo, da to storite brez kode, saj to omogoča tudi strežniške validacije. Na strani za prilagajanje gradnika si oglejte razdelek "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Nastavitev sloga glasovanja na strani za prilagajanje gradnika, ki ponuja puščice gor/dol ali glasovanje s srcem'; title='Spremeni slog glasovanja' app-screenshot-end]

Glasovanje je mogoče tudi onemogočiti, glejte `Disable Voting` nad možnostmi sloga.

---