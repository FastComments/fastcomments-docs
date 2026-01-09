[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Kommentering kan låses, så der ikke kan efterlades nye kommentarer eller stemmer ved at sætte readonly-flaget til true.

Kommentarer vil også ikke kunne redigeres eller slettes.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Dette kan tilpasses uden kode, på widget-tilpasningssiden, for et helt domæne, eller en side:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Opdatering!

Fra november 2022 kan tråde låses eller låses op **live** af administratorer og moderatorer via tre-punkts-menuen over svarområdet.

Dette vil forhindre nye kommentarer, mens stemmegivning stadig er tilladt, og brugere kan slette deres kommentarer, hvis de ønsker det, mens `readonly` ikke tillader disse ting. 

Dette svarer til feltet `isClosed` i `Page` API'en.