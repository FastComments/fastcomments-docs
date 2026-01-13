Du kan opleve, at vores Analytics-metrikker viser lidt andre tal end f.eks. Google Ads © eller lignende produkter.

For websteder med én kommentar-widget pr. side er tallene fra FastComments Analytics meget nøjagtige, og hvis de er forkerte, vil de være **lavere** end den faktiske værdi, men ikke mere.

Hvis du har en SPA, kan du opleve, at FastComments Analytics-tallene er højere end dem, der rapporteres af dine marketingprodukter. Dette skyldes, at marketingproduktet muligvis kun sporer, når siden ikke er indlæst, men ikke hver gang en bruger gør noget på siden, der kan udløse visning af en ny kommentartråd, hvilket ville tælle som en sideindlæsning for FastComments Analytics.

#### Teknisk information

FastComments Analytics sporer hver sideindlæsning og er ikke afhængig af tilfældighed som en optimering. Hver sideindlæsning resulterer i, at en tæller i hukommelsen opdateres i hver tråd på hver server, som derefter periodisk gemmes i databasen via en atomisk operation.
