[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Po podrazumevanoj postavci, funkcionalnosti formatiranja u FastComments se vrše dodavanjem vidljivih HTML tagova kao što su `<b></b>` oko vašeg teksta. Klikom na traku sa alatkama ili korišćenjem prečica to se radi umesto vas. Međutim, neke zajednice možda žele da se odluče za korišćenje formatiranja bez vidljivih tagova. Ovo se naziva omogućavanjem WYSIWYG (ono što vidiš je ono što dobiješ) editora. Ovaj editor izgleda tačno isto kao podrazumevani, osim što učitava dodatni kod koji omogućava korisnicima da podebljaju, podvuču itd. svoj tekst bez vidljivih tagova.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Ovo se takođe može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte opciju "Omogući napredno formatiranje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]