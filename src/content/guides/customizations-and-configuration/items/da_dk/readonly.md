[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Kommentar kan låses, så ingen nye kommentarer eller stemmer kan afgives ved at sætte readonly-flaget til true.

Kommentarer vil også ikke kunne redigeres eller slettes.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Gør kommentartråden skrivebeskyttet'; code-example-end]

Dette kan tilpasses uden kode på widget-tilpasningssiden, for et helt domæne eller en side:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Indstillingen for at forhindre nye svar på widget-tilpasningssiden, som låser en tråd for et domæne eller en side'; title='Gør kommentartråden skrivebeskyttet' app-screenshot-end]

## Update!

Fra november 2022 kan tråde låses eller låses op **live** af administratorer og moderatorer via tre-punkts-menuen over svarområdet.

Dette vil forhindre nye kommentarer, mens stemmer stadig er tilladt, og brugere kan slette deres kommentarer, hvis de ønsker det, mens `readonly` ikke tillader disse ting. 

Dette svarer til `isClosed`-feltet i `Page`-API'en.

---