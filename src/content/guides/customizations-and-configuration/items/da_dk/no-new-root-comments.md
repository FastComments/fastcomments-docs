[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Indstilling af `noNewRootComments` til `true` får widgeten til at skjule det øverste svarområde, men tillader stadig brugere at svare
på underkommentarer. Du kan for eksempel sætte dette betinget ved sideindlæsning, så kun nogle brugere kan efterlade kommentarer på øverste niveau.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]