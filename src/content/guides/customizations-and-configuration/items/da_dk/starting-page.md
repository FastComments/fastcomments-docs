[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Når der hentes og gengives kommentarer, skal kommentarwidgeten vide, hvilken side den skal starte på. Som standard starter den med
den første side og gengiver kun den side.

Hvis ønsket, kan den præcise side, der skal gengives, sendes til kommentarwidgeten som indstillingen *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Bemærk, at sidetal begynder ved nul, så ovenstående eksempel gengiver den anden side.

---