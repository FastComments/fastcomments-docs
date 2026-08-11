[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, funkcionalnosti formatiranja u FastComments se ostvaruju dodavanjem vidljivih oznaka sidra poput `<b></b>` oko vašeg teksta. Klikanje na alatnu traku ili korištenje prečaca to radi za vas. Međutim, neke zajednice mogu željeti koristiti formatiranje bez oznaka sidra. To se naziva omogućavanje WYSIWYG (what you see is what you get) uređivača. Ovaj uređivač izgleda točno isto kao zadani, osim što učitava dodatni kod koji korisnicima omogućuje podebljanje, podcrtavanje, itd. njihovog teksta bez vidljivih oznaka sidra.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Omogućavanje WYSIWYG uređivanja'; code-example-end]

Ovo se također može učiniti bez koda. Na stranici za prilagodbu widgeta, pogledajte opciju "Enable Advanced Formatting" opciju.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Stranica za prilagodbu widgeta s označenim potvrdnim okvirom Enable Advanced Formatting za uključivanje WYSIWYG uređivača'; title='Omogući WYSIWYG' app-screenshot-end]