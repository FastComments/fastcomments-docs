## Fejlfinding

**"Du har ikke tilladelse" når du opretter forbindelse.** Den indloggede bruger er ikke en API‑admin på kontoen.  
Bed kontoejeren om at give API‑tilladelse på siden Brugere, eller opret forbindelse som ejer.

**Forbindelsen er mærket med det forkerte websted.** Samtykkesiden forbinder den konto, du var logget ind  
på på det tidspunkt. Afbryd i Zapier, skift konto i FastComments‑instrumentbrættet, og opret forbindelse igen.

**Begivenheder stoppede med at ankomme.** Tjek siden Webhooks i instrumentbrættet. Et abonnement, hvis endpoint fejlede i seks dage, deaktiveres automatisk og viser årsagen. Genaktiver det der, eller slå Zap’en fra og til igen.  
Hvis abonnementet helt mangler, har nogen slettet det; at slå Zap’en fra og til genskaber det.

**Zapier siger, at kontoen skal genoprettes.** Forbindelsen blev tilbagekaldt fra siden Tilsluttede apps,  
brugeren som godkendte den mistede API‑tilladelsen, eller kontoen blev slettet. Opret forbindelse igen fra Zapier.

**En handling fejler med "har ikke skriveadgang".** Forbindelsen blev godkendt med kun læse‑tilladelse.  
Opret forbindelse igen og godkend begge tilladelser.

**Rate‑begrænsninger og kreditter.** Handlinger og søgninger bruger API‑kreditter fra din plan og er underlagt de samme hastighedsbegrænsninger som REST‑API’en. Udløsere bruger ingen. En Zap, der rammer en grænse, genforsøges af Zapier  
efter den ventetid, som FastComments rapporterer.

**Domæne‑dropdown‑listen er tom.** Domæner vises, når de er konfigureret på siden Domæner i FastComments‑instrumentbrættet.  
Lad feltet stå tomt for at modtage begivenheder for alle domæner.