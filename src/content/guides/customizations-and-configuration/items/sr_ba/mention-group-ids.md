---
[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista ID-eva koja će se koristiti za automatsko dovršavanje **@mentions**. Korisno kada želite spriječiti označavanje korisnika koji nemaju zajedničke grupe.

Kada je navedeno, u automatskom dovršavanju će biti prikazani samo korisnici iz drugih grupa nakon što otkucate karakter `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---