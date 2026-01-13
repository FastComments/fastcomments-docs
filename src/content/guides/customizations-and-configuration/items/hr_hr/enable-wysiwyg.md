[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, funkcionalnosti formatiranja u FastComments postižu se dodavanjem vidljivih anchor tagova poput `<b></b>` oko vašeg teksta. Klikom na alatnu traku
ili korištenjem prečaca to se radi za vas. Međutim, neke zajednice možda žele odlučiti se za korištenje formatiranja bez anchor tagova. To se naziva omogućavanjem
WYSIWYG (ono što vidiš je ono što dobiješ) uređivača. Ovaj uređivač izgleda točno isto kao zadani, osim što učitava neki
dodatni kod koji korisnicima omogućuje podebljavanje, podcrtavanje i slično njihovog teksta bez vidljivih anchor tagova.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Ovo se također može učiniti i bez koda. Na stranici za prilagodbu widgeta potražite opciju "Omogući napredno formatiranje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]