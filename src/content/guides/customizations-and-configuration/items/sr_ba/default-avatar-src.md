[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Kada korisnik prvi put komentariše pomoću FastComments-a, pokušaćemo preuzeti njihov avatar sa <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Međutim, ako ne pronađemo avatar, ili korisnik nikada ne postavi jedan u svom nalogu, prikazujemo statičku podrazumijevanu sliku avatara.

Da biste naveli vlastitu statičku sliku avatara, možete koristiti podešavanje *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte odjeljak "Podrazumijevani avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Napomena: Definisanje avatara za određenog korisnika, kao npr. kod SSO-a, obuhvaćeno je u zasebnom odjeljku.