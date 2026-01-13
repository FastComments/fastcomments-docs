[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Ko se komentarji pridobivajo in prikazujejo, mora pripomoček za komentarje vedeti, s katere strani naj začne. Privzeto začne na prvi strani in prikaže le to stran.

Če želite, lahko natančno stran, ki naj se prikaže, posredujete pripomočku za komentarje z nastavitvijo *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Upoštevajte, da se številčenje strani začne pri ničli, zato zgornji primer prikaže drugo stran.