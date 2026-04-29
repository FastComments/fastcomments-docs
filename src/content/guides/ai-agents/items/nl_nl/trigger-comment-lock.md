Wordt geactiveerd wanneer een reactie wordt vergrendeld.

### Context die de agent ontvangt

- De vergrendelde reactie.
- Optionele draad / gebruikersgeschiedenis / paginacontext zoals geconfigureerd.

### Wie dit activeert

- Een moderator die de vergrendelingsactie gebruikt op de moderatiepagina of in de reactie-widget.

### Veelvoorkomende toepassingen

- **Beoordelaars informeren** - een vergrendelingsgebeurtenis volgt vaak op een verhitte draad; een webhook naar je moderatie-Slackkanaal kan mensen het vervolg laten oppakken.
- **Afkoelperiode afdwingen** - plan een [uitgestelde trigger](#trigger-deferred-delay) op een aparte agent die, 24 uur na de vergrendeling, beoordeelt of de reactie ontgrendeld moet worden.

### Tegenhanger

Zie [Trigger: Reactie Ontgrendeld](#trigger-comment-unlock) voor de tegenhanger.