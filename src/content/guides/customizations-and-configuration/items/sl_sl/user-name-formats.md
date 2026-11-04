Privzeto FastComments prikaže uporabniško ime, kot ga je uporabnik vnesel, ali pa tako, kot nam je bilo posredovano prek SSO.

Vendar je lahko zaželeno ime uporabnika maskirati ali prikazati na drugačen način. Na primer, če se uporabnik imenuje Allen Rex, morda
želite prikazati le "Allen R.".

To je mogoče storiti brez kode v uporabniškem vmesniku za prilagoditev pripomočka, pod nastavitvijo `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Odprt spustni meni Commenter Name Format z možnostmi, kot so Capitalize, Last Initial in All Initials'; title='Sprememba formata imena' app-screenshot-end]

Na voljo so naslednji formati:

- Capitalize (uporabnik Example User se prikaže kot Example User)
- Last Initial (uporabnik Example User se prikaže kot Example U.)
- All Initials (uporabnik Example User se prikaže kot E. U.)
- Prikaži "Anonymous"

Učinek spremembe je takojšen. Uporabniki bodo še vedno videli svoje polno uporabniško ime na vrhu območja za komentarje, zase, njihovi komentarji pa bodo prikazali
spremenjeno uporabniško ime.

Uporabniška imena so maskirana na strani strežnika, da se uporabniki zaščitijo.