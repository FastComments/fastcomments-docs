---
[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Komentiranje je mogoče zakleniti, tako da z nastavitvijo zastavice readonly na true ni mogoče oddajati novih komentarjev ali glasov.

Komentarjev prav tako ne bo mogoče urejati ali brisati.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

To je mogoče prilagoditi brez kode, na strani za prilagajanje widgeta, za celotno domeno ali stran:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Posodobitev!

Od novembra 2022 lahko administratorji in moderatorji teme zaklenejo ali odklenijo **v živo** preko menija s tremi pikami nad območjem za odgovor.

To bo preprečilo nove komentarje, hkrati pa bo še vedno omogočalo glasovanje in uporabnikom dovoljevalo brisanje njihovih komentarjev, če si to želijo, medtem ko `readonly` tega ne omogoča. 

To ustreza polju `isClosed` v API-ju `Page`.

---