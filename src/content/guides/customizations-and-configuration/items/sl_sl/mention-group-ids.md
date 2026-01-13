[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Seznam ID-jev za uporabo pri samodokončanju **@omemb**. Uporabno, če želite preprečiti označevanje uporabnikov, kadar nimajo prekrivajočih se skupin.

Če je določeno, bodo v samodokončanju po vnosu znaka `@` prikazani le uporabniki iz drugih skupin.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]