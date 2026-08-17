[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Kada korisnik prvi put komentariše pomoću FastComments, pokušaćemo da preuzmemo njegov avatar sa <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

Međutim, ako ne pronađemo avatar, ili korisnik nikada ne postavi jedan u svom nalogu, prikazaćemo statičku podrazumevanu sliku avatara.

Da biste naveli svoju statičku sliku avatara, možete koristiti podešavanje *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Zameni podrazumevani avatar'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak „Default Avatar“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Odeljak podrazumevanog avatara na stranici za prilagođavanje widgeta, gde postavljate URL rezervne slike avatara'; title='Prilagođavanje podrazumevanog avatara' app-screenshot-end]

Napomena da definisanje avatara za određenog korisnika, kao što je SSO, je pokriveno u svom odeljku.