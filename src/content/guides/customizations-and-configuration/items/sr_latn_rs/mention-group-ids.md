[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Lista ID-jeva koja se koristi za automatsko dopunjavanje **@mentions**. Korisno kada želite sprečiti označavanje korisnika koji nemaju zajedničke grupe.

Kada je naveden, u automatskom dopunjavanju će, nakon kucanja karaktera `@`, biti prikazani samo korisnici iz drugih grupa.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---