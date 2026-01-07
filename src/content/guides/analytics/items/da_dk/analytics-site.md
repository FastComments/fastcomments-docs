Site Analytics, eller bare kaldet `Analytics` i dashboardet, giver et overblik over, hvordan dit fællesskab bruger FastComments på tværs af alle dine domæner.

FastComments tilbyder nogle unikke funktioner, som mange andre platforme ikke tilbyder, som **live** rapportering af online brugere på hver side og sortering af sider efter antallet af online brugere. For at gøre dette skal du blot besøge [Analytics Page](https://fastcomments.com/auth/my-account/analytics) og klikke på `Sort by users online` under `Top Pages`.

Både det samlede `Users Online` og `Top Pages` metrikker er live og rapporteres uden forsinkelse.

`Top Pages` vil som standard sortere efter antallet af kommentarer på hver side.

Endelig gives der en opdeling for samlede metrikker på tværs af din lejer, per dag, over tid for:

- Page Loads
  - Dette er antallet af gange, en bruger åbnede en side, der indeholder en eller flere FastComments-widgets. Hvis siden indeholder flere widgets, vil dette tal blive forøget med antallet af widgets på den side. Hvis du har en SPA, vil dette tal blive forøget, hver gang applikationen åbner en ny kommentartråd. Dette gælder også for React Native-biblioteket.
  - Denne metrik bruges også til faktureringsformål i Flex-planerne.
- Comments Left
  - Dette inkluderer alle kommentarer, uanset verifikations- eller godkendelsesstatus, eller om de er spam eller ej.
- Votes Left
  - Dette er for antallet af afgivne stemmer. Det vil kun tælle verificerede stemmer, medmindre anonym afstemning er aktiveret.
- Accounts Created
  - Denne metrik er for, når en ny SSO-bruger tilføjes, eller en kommentator kommenterer med FastComments for første gang ved hjælp af dit websted.

Disse metrikker er næsten realtid, med forsinkelse på op til et minut.
