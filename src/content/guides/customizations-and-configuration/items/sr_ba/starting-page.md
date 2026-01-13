[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Kada se preuzimaju i renderuju komentari, widget za komentare mora znati sa koje stranice da počne. Podrazumevano počinje od prve stranice i prikazuje samo tu stranicu.

Ako želite, tačnu stranicu koja treba da se prikaže možete proslediti widgetu za komentare putem podešavanja *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Imajte na umu da brojevi stranica počinju od nule, pa gornji primer prikazuje drugu stranicu.

---