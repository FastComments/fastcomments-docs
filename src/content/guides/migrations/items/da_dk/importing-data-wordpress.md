Vores [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) har en kraftfuld UI-baseret importmekanisme. Ved installation af pluginet
guide vi dig gennem at koble din WordPress-installation til FastComments og kopiere dine eksisterende kommentardata over.

**Dette gøres uden at kopiere eller downloade noget manuelt.**

Migrationsprocessen vil blive vist dig via UI'et under migrationen. De fleste migrationer tager kun et par minutter.

Mekanismen er designet til ikke at lægge for stor belastning på din WordPress-installation under migrationen.

Hvis du flytter dit websted væk fra WordPress, kan du importere en WordPress XML- eller CSV-eksport i stedet for at bruge pluginet. Se
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Firewalls

For at den automatiserede WordPress-opsætning kan fungere, er vi nødt til at lave kald til din WordPress-installation.
Firewalls som Cloudflare kan blokere os og få integrationen til at fejle. I sådanne tilfælde kan [vi give
dig](https://fastcomments.com/auth/my-account/help) et sæt IP-adresser, som du kan whiteliste for integrationen.

### Dataejerskab

I forbindelse med vores WordPress-migration synkroniseres alle nye eller opdaterede kommentardata automatisk tilbage til din WordPress-installation
i baggrunden. Det betyder, at selvom kommentarerne serveres af FastComments selv for at aflaste din WordPress-implementering,
gemmer vi dem **også** i din database som backup. Det betyder også, at hvis du ønsker at skifte væk fra FastComments, er dine data
allerede migreret og opdaterede.