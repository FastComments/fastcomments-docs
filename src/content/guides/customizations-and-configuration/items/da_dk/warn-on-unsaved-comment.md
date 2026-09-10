[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Som standard, hvis en bruger skriver en kommentar og derefter opdaterer siden, lukker fanen eller navigerer væk før indsendelse, går udkastet tabt uden varsel.

Indstilling af **warnOnUnsavedComment** til true får browseren til at bede brugeren bekræfte, før de forlader siden, mens enhver kommentarboks eller en igangværende redigering stadig indeholder tekst. Når kommentaren er indsendt, ryddes teksten, så der ikke vises nogen prompt.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Advarsel om ikke-gemt kommentar'; code-example-end]

Prompten bruger browserens egen dialog. Moderne browsere viser deres egen formulering og ignorerer tilpasset tekst, så beskeden kan ikke tilpasses.

Denne indstilling indlæser en lille udvidelse på efterspørgsel, så den tilføjer intet til widgeten for sider, der ikke aktiverer den.