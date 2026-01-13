[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Privzeto se oblikovne funkcionalnosti v FastComments izvajajo z dodajanjem vidnih oznak, kot je `<b></b>`, okoli vašega besedila. Klikanje orodne vrstice
ali uporaba bližnjic to naredi za vas. Vendar pa se nekatere skupnosti morda želijo odločiti za uporabo oblikovanja brez vidnih oznak. To se imenuje vklop
WYSIWYG (kar vidiš, to dobiš) urejevalnika. Ta urejevalnik izgleda enako kot privzeti, razen da naloži nekaj
dodatne kode, ki uporabnikom omogoča krepko, podčrtano itd. oblikovanje besedila brez vidnih oznak.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

To je mogoče storiti tudi brez kode. Na strani za prilagajanje widgeta poiščite možnost "Omogoči napredno oblikovanje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]