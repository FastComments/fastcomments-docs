[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Popis id-ova za korištenje u automatskom dovršavanju **@mentions**. Korisno kada želite spriječiti označavanje korisnika koji nemaju zajedničke grupe.

Kada je navedeno, u automatskom dovršavanju će biti ponuđeni samo korisnici iz drugih grupa nakon upisivanja znaka `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---