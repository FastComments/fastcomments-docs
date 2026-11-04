Hvert WordPress-site kræver sin egen FastComments-konto eller tenant. WordPress nummererer indlæg separat på hvert site, så indlæg 5 på ét site og indlæg 5 på et andet
ville dele en enkelt kommentartråd, hvis begge var forbundet til samme konto. Af den grund nægter plugin'ets opsætning at forbinde et site, når kontoen allerede har et andet forbundet.

[Du kan se dine aktive integrationer her.](https://fastcomments.com/auth/my-account/manage-data/integrations)

For at køre flere WordPress-sites skal du oprette en [tenant](https://fastcomments.com/auth/my-account/tenants) til hvert af dem, skifte til den og derefter forbinde sitet fra plugin'ets opsætningsside. Tenants er inkluderet i planer med white labeling. Du kan også oprette en separat FastComments-konto til hvert site med en anden e-mailadresse til hver.

Hvis et site er flyttet, f.eks. fra staging til produktion, skal du deaktivere plugin'et på det gamle site og fjerne det gamle site fra dine aktive integrationer. Derefter forbinder du det nye. Kommentarer, der allerede er synkroniseret fra det gamle site, forbliver på kontoen, og de vises på ethvert indlæg med samme ID på det nye site.

Du kan bruge samme FastComments-konto med en enkelt WordPress-installation, der spænder over flere domæner og har det samme indhold på disse domæner.