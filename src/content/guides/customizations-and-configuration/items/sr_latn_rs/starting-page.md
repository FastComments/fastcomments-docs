[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Prilikom preuzimanja i renderovanja komentara, widget za komentare mora znati od koje stranice da počne. Podrazumevano, počinje sa
prvom stranicom i prikazuje samo tu stranicu.

Ako želite, tačnu stranicu koja će se renderovati možete proslediti widgetu za komentare kao podešavanje *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Imajte na umu da brojevi stranica počinju od nule, pa gornji primer prikazuje drugu stranicu.

---