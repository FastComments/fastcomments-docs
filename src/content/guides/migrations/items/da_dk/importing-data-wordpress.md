Vores [WordPress-plugin](https://wordpress.org/plugins/fastcomments/) har en kraftfuld importmekanisme baseret på brugergrænsefladen. Når du installerer plugin'et,
vil det guide dig gennem at forbinde din WordPress-installation med FastComments og kopiere dine eksisterende kommentar-data over.

**Dette sker uden at skulle kopiere eller downloade noget manuelt.**

Migreringsprocessen vil blive vist for dig via brugergrænsefladen under migreringen. De fleste migrationer tager kun et par minutter.

Mekanismen er designet til ikke at påføre din WordPress-installation overdreven belastning under migreringen.

### CloudFlare & Firewalls

For at den automatiserede WordPress-opsætning kan fungere, skal vi foretage kald til din WordPress-installation.
Firewalls som Cloudflare kan blokere os og få integrationen til at fejle. I sådanne tilfælde, [kan vi give dig](https://fastcomments.com/auth/my-account/help) et sæt IP-adresser, som du kan føje til tilladelisten for integrationen.

### Dataejerskab

I forbindelse med vores WordPress-migrering synkroniseres alle nye eller opdaterede kommentar-data automatisk tilbage til din WordPress-installation bag kulisserne. Det betyder, at mens kommentarerne leveres af FastComments selv for at aflaste din WordPress-installation, gemmer vi dem **også** i din database som backup. Det betyder også, at hvis du ønsker at skifte væk fra FastComments, er dine data allerede migreret og opdaterede.