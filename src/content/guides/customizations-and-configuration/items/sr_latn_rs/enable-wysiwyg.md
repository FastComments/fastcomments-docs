[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Podrazumevano, funkcionalnosti formatiranja u FastComments se ostvaruju dodavanjem vidljivih anchor tagova poput `<b></b>` oko vašeg teksta. Klikom na alatnu traku ili korišćenjem prečica se to radi za vas. Međutim, neke zajednice mogu želeti da koriste formatiranje bez anchor tagova. Ovo se naziva omogućavanje WYSIWYG (what you see is what you get) uređivača. Ovaj uređivač izgleda identično podrazumevanom, osim što učitava dodatni kod koji omogućava korisnicima da podebljaju, podvlače, itd. svoj tekst bez vidljivih anchor tagova.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Omogućavanje WYSIWYG uređivanja'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte opciju "Enable Advanced Formatting" opciju.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Stranica za prilagođavanje widgeta sa čekiranim poljem Omogući napredno formatiranje za uključivanje WYSIWYG uređivača'; title='Omogući WYSIWYG' app-screenshot-end]