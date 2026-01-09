[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Kada korisnik prvi put komentira putem FastComments pokušat ćemo dohvatiti njihov avatar s <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Međutim, ako ne pronađemo avatar, ili korisnik nikada ne postavi jedan u svom računu, prikazujemo statičnu zadanu sliku avatara.

Za određivanje vlastite statične slike avatara možemo koristiti postavku *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta pogledajte odjeljak "Zadani avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Imajte na umu da je definiranje avatara za pojedinog korisnika, kao npr. pomoću SSO-a, obuhvaćeno u zasebnom odjeljku.

---