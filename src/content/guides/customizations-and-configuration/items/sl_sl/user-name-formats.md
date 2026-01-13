Privzeto bo FastComments prikazal uporabnikovo ime tako, kot ga je vnesel, ali tako, kot je bilo posredovano prek SSO.

Včasih pa je zaželeno zamaskirati ali prikazati uporabnikovo ime na drugačen način. Na primer, če je uporabnikovo ime Allen Rex, morda želite prikazati samo "Allen R.".

To je mogoče narediti brez kode v vmesniku za prilagajanje gradnika (Widget Customization UI), v nastavitvi z imenom `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Na voljo so naslednji formati:

- Velike začetnice (prikaže primer uporabnika kot Example User)
- Zadnja inicialka (prikaže Example User kot Example U.)
- Vse inicialke (prikaže Example User kot E. U.)
- Prikaži "Anonimen"

Učinek spremembe je takojšen. Uporabniki bodo še vedno sami videli svoje polno uporabniško ime na vrhu območja za komentarje, vendar bodo njihovi komentarji prikazovali spremenjeno uporabniško ime.

Uporabniška imena so zaradi zaščite uporabnikov zamaskirana na strežniški strani.