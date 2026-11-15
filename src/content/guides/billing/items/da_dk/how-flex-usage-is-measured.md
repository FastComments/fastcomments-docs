En Flex-faktura er en liste over linjeposter, én pr. brugstype, hvor hver viser mængden og taksten, f.eks. `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Taksterne findes på [prissætningssiden](https://fastcomments.com/traffic-pricing) og på din [Faktureringsinfo](https://fastcomments.com/auth/my-account/billing-info)-side. Dette afsnit forklarer, hvad hver mængde betyder.

### Tælles over måneden

Disse nulstilles hver måned og tælles fra den første til den sidste dag i kalendermåneden (UTC):

- **Page Loads**: antallet af gange, en kommentarwidget eller feed er blevet indlæst på dine sider. Hver indlæsning af kommentarwidgeten tæller, uanset hvor mange kommentarer den viser. Der opkræves intet gebyr pr. kommentartråd eller pr. side.
- **API Credits**: kald til REST API'en med din API-nøgle.
- **Small Widgets Credits**: indlæsninger af de små widgets, såsom kommentartællinger og badges.
- **LLM Tokens**: tokens brugt af [AI Agents](/guide-ai-agents.html) og andre LLM-funktioner.

### Tælles ved månedens udgang

Disse er et øjebliksbillede, der tages, når fakturaen oprettes:

- **Comments Stored**: det samlede antal kommentarer på din konto, ikke kun dem, der er tilføjet denne måned. Sletning af kommentarer reducerer tallet, bortset fra at en slettet kommentar med svar bevares som en pladsholder, så svarene stadig giver mening.
- **SSO Users**: det samlede antal SSO-brugere, som dit site har oprettet, inklusive personer, der kommenterede én gang for længe siden. Hvis dette er den største del af din regning, så spørg supporten om fakturering pr. månedlige aktive brugere i stedet, hvor kun brugere, der loggede ind i løbet af måneden, tæller.
- **Moderators**, **Admins**, **SSO Moderators** og **SSO Admins**: pladser, som er dækket i [Admin- og Moderator-pladser](#admin-and-moderator-seats). Den første admin er gratis.
- **Domains**: sites i din konto. Det første site er gratis, og hvert ekstra site faktureres pr. måned, så længe det er i kontoen. Et subdomæne tæller kun, hvis du tilføjer det som sit eget site; et enkelt topdomæne dækker dets subdomæner. Se [Flere Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: underkonti, der administreres af denne konto, hvis der er nogen, plus deres egen brug som separate linjer.

### Minimum

Hvis linjerne tilsammen udgør mindre end Flex-minimummet, efterfyldes fakturaen til minimummet. Et lille site med én admin og ét domæne betaler kun minimummet.

### Forhåndsvisning af fakturaen

Siden [Faktureringsanalyse](https://fastcomments.com/auth/my-account/analytics/billing) viser hver linje i den næste faktura med dagens tal, så der ikke er nogen overraskelser. Sideindlæsnings-, API- og small widget-tællingerne kan ligge op til et minut bagud; resten er live.

### Andre valutaer

Flex-forbrug prissættes i amerikanske dollars. Hvis din fakturavaluta er noget andet, konverteres hver linje til valutakursen på den dag, fakturaen oprettes, så det samme forbrug kan koste lidt forskellige beløb fra måned til måned. Se [Valutaer](#currencies).