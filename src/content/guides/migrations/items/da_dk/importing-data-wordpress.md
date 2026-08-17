Vores [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) har en kraftfuld UI-baseret importmekanisme. Når du installerer plugin'et,
vil den guide dig gennem at forbinde din WordPress-installation med FastComments og kopiere dine eksisterende kommentardata over.

**Dette gøres uden at kopiere eller downloade noget manuelt.**

Migrationsprocessen vil blive vist for dig via UI'en under migreringen. De fleste migrationer tager kun et par minutter.

Mekanismen er designet til ikke at belaste din WordPress-installation unødigt under migreringen.

### CloudFlare & FireWalls

For at den automatiserede WordPress-opsætning kan fungere, skal vi foretage kald til din WordPress-installation.
Firewalls som Cloudflare kan blokere os og få integrationen til at fejle. I sådanne tilfælde, [kan vi give
dig](https://fastcomments.com/auth/my-account/help) et sæt IP-adresser, som du kan whitelist for integrationen.

### Data Ownership

I tilfældet med vores WordPress-migration synkroniseres alle nye eller opdaterede kommentardata automatisk tilbage til din WordPress-installation
bag kulisserne. Det betyder, at mens kommentarerne leveres af FastComments selv for at reducere belastningen på din WordPress-implementering,
vi **også** gemmer dem i din database som en backup. Det betyder også, at hvis du ønsker at skifte væk fra FastComments, er dine data
allerede migreret og opdateret.