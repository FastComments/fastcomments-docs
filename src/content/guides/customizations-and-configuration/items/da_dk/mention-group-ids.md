[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

En liste med id'er til brug for **@mentions** autokomplettering. Nyttigt, når du vil forhindre at tagge brugere, hvis de ikke har overlappende grupper.

Når det er angivet, vil kun brugere i andre grupper blive vist i autokompletteringen efter at have skrevet `@`-tegnet.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---