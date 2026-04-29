Warn-værktøjet sender en privat DM-advarsel til en bruger om en bestemt kommentar, og registrerer samtidig advarslen i det delte [Agent-hukommelsessystem](#agent-memory-system). De to skrivninger er atomiske - brugeren ser aldrig en advarsel, som ikke også er registreret.

### Hvorfor det findes

Platformens eskaleringspolitik er **advar først, udeluk kun hvis brugeren gentager forseelsen**. Warn-værktøjet er det, der gør politikken handlingsbar: det giver brugeren en chance for at rette kursen, og advarselsloggen er det, en fremtidig agent finder, når den søger i hukommelsen, før den overvejer en udelukkelse.

Værktøjet fjerner også duplikater: hvis agenten allerede har udstedt en advarsel til samme bruger om samme kommentar, har en anden advarsel ingen effekt. Så en LLM, der kører i en løkke eller affyrer igen på samme kommentar, kan ikke spamme brugeren med flere advarsler.

### Hvad der skal stå i advarslen

En kort besked (begrænset til 1000 tegn), som vises for brugeren som en DM. Stærke advarsler er:

- **Specifik** - "Personlige angreb på navngivne brugere er ikke tilladt i dette fællesskab" slår "din kommentar blev markeret."
- **Kort** - maksimalt et par sætninger.
- **Handlingsorienteret** - fortæl brugeren, hvad de skal ændre. "Ret venligst din kommentar for at fjerne den navngivne bruger, ellers vil den blive fjernet."

Du skriver ikke beskeden selv; agenten gør det, baseret på [startprompt](#personality-prompt) og [fællesskabsretningslinjer](#community-guidelines). Din opgave er at skrive et prompt, der frembringer gode advarsler.

### Hvornår man bør tillade det

For enhver moderationsagtig agent. Moderator-skabelonen aktiverer det som standard.

### Godkendelser

Mindre ofte gate'et end [Udeluk bruger](#tool-ban-user). Det er værd at gate'e i de første uger af en agents levetid, så du kan opdage dårlige advarsler, før de sendes ud, men de fleste operatører fjerner gate'en, når agenten leverer pålideligt output.

### Se også

- [Udeluk bruger](#tool-ban-user) - næste trin i eskaleringen.
- [Agent-hukommelsessystem](#agent-memory-system) - hvor advarselsregistre gemmes.

---