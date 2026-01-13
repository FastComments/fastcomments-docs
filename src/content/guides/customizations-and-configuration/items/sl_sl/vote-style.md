[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Privzeto bo FastComments prikazal možnosti glasovanja kot puščice gor in dol, kar uporabnikom omogoča, da komentar označijo z glasom gor ali dol.

Vendar je mogoče spremeniti slog orodne vrstice za glasovanje. Trenutne možnosti so privzeti gumbi Gor/Dol ali mehanizem za glasovanje v obliki srca.

Uporabljamo zastavico **voteStyle** na naslednji način:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Močno priporočamo, da to storite brez kode, saj tako omogočite tudi preverjanja na strežniški strani. Na strani za prilagajanje widgeta si oglejte razdelek "Slog glasovanja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Glasovanje je mogoče tudi onemogočiti, glejte `Disable Voting` zgoraj pri možnostih sloga.

---