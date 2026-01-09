[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Kada korisnik prvi put komentariše koristeći FastComments, pokušaćemo da dohvatimo njegov avatar sa <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Međutim, ako ne pronađemo avatar, ili korisnik nikada ne postavi jedan na svom nalogu, prikazaćemo statičku podrazumevanu sliku avatara.

Da biste odredili sopstvenu statičku sliku avatara, možete koristiti podešavanje *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak "Podrazumevani avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Imajte na umu da je definisanje avatara za određenog korisnika, na primer pomoću SSO, objašnjeno u posebnom odeljku.

---