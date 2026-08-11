[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Kada korisnik prvi put komentira putem FastComments, pokušat ćemo dohvatiti njegov avatar s <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Međutim, ako ne pronađemo avatar ili korisnik nikada ne postavi avatar u svom računu, prikazat ćemo statičku zadanu sliku avatara.

Za određivanje vlastite statičke slike avatara možete koristiti postavku *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Zamijeni Zadani Avatar'; code-example-end]

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta, pogledajte odjeljak "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Odjeljak Zadani Avatar na stranici prilagodbe widgeta, gdje postavljate URL rezervne slike avatara'; title='Prilagodba Zadanog Avatara' app-screenshot-end]

Napomena: definiranje avatara za određenog korisnika, npr. putem SSO, obrađeno je u vlastitom odjeljku.