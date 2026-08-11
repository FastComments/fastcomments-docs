[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentiranje je mogoče zakleniti, tako da z nastavitvijo zastavice readonly na true ne morejo biti objavljeni novi komentarji ali glasovi.

Komentarji tudi ne bodo mogli biti urejani ali izbrisani.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Zaklepanje niti komentarjev'; code-example-end]

To je mogoče prilagoditi brez kode, na strani za prilagajanje gradnika, za celotno domeno ali stran:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Nastavitev preprečevanja novih odgovorov na strani za prilagajanje gradnika, ki zaklene nit za domeno ali stran'; title='Zaklepanje niti komentarjev' app-screenshot-end]

## Posodobitev!

Od novembra 2022 je mogoče niti zakleniti ali odkleniti **v živo** s strani skrbnikov in moderatorjev prek menija s tremi pikami nad območjem odgovora.

To bo preprečilo nove komentarje, hkrati pa bo še vedno omogočalo glasovanje in omogočalo uporabnikom, da po želji izbrišejo svoje komentarje, medtem ko `readonly` teh stvari ne dovoljuje. 

To ustreza polju `isClosed` v API-ju `Page`.