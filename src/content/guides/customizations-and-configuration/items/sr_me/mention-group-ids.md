[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista ID-ova koja se koristi za automatsko dovršavanje **@mentions**. Korisno kada želite spriječiti označavanje korisnika koji nemaju zajedničke grupe.

Kada je navedeno, u automatskom dovršavanju biće prikazani samo korisnici iz drugih grupa nakon što otkucate znak `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---