If du flytter dit site fra WordPress og ønsker FastComments på det nye site, behøver du ikke WordPress‑pluginet. Eksporter dine kommentarer fra
WordPress, og upload derefter filen på [Import side](https://fastcomments.com/auth/my-account/manage-data/import) i FastComments‑dashboardet.

Vi understøtter to WordPress‑eksportformater.

### WordPress XML (Anbefalet)

Dette er filen fra WordPress' indbyggede eksportværktøj, så der er ikke brug for et ekstra plugin.

1. I din WordPress‑admin, gå til `Tools -> Export`.
2. Vælg `All content` og klik på `Download Export File`.
3. På FastComments [Import side](https://fastcomments.com/auth/my-account/manage-data/import), vælg `WordPress (.xml)` og upload filen.

Hver kommentar er knyttet til URL’en for det indlæg, den blev efterladt på, hvilket allerede er i filen.

Importen bevarer forfatternavnet, e‑mail og hjemmeside, datoen, indholdet, svartrådene og om kommentaren var godkendt. Kommentator‑avatars hentes fra Gravatar. Stemmer er ikke en del af dette format.

### WordPress CSV

Dette er filen fra [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Installer pluginet i din WordPress‑admin og eksporter dine kommentarer som CSV.
2. Erstat hver `comment_post_ID`‑værdi med indlæggets URL.
3. På FastComments [Import side](https://fastcomments.com/auth/my-account/manage-data/import), vælg `WordPress (.csv)` og upload filen.

Hver kommentar er knyttet til kolonnen `comment_post_ID`. WordPress udfylder denne kolonne med indlæggets ID, og dit nye site har ikke WordPress‑indlægs‑ID’er, så trin 2 erstatter den med URL’en.

Importen bevarer forfatternavnet, e‑mail og hjemmeside, datoen, indholdet, svartrådene og om kommentaren var godkendt. Kommentator‑avatars hentes fra Gravatar. Den bevarer også WordPress' spam‑flag og wpDiscuz‑likes og -dislikes, når filen indeholder dem.

### Matche kommentarer til dine nye sider

Hvis dit nye site beholder de samme URL’er som dit WordPress‑site, vises kommentarerne på de tilsvarende sider uden ekstra opsætning.

Hvis domænet ændres, kør [Domain Migration værktøj](/guide-migrations.html#migrating-domains) efter importen. Hvis individuelle side‑URL’er ændres, kan du
[migrer hver side](/guide-migrations.html#migrating-pages) fra den gamle URL til den nye.

For masse‑side‑migrationer, såsom at fjerne domænet fra den værdi, du sender til kommentar‑widget'ens [urlId](/guide-customizations-and-configuration.html#url-id)
felt, [åbn en support‑ticket](https://fastcomments.com/auth/my-account/help) og vi vil håndtere det for dig.

### Før du skifter

Du kan køre importen så mange gange, du vil. Gen‑import af den samme fil [skaber ikke dubletter](/guide-migrations.html#importing-data), så du kan
importere én gang for at teste det nye site, og derefter importere igen med dine seneste kommentarer lige før du skifter over.

For eksportfiler større end 1 GB, [kontakt support](https://fastcomments.com/auth/my-account/help).

For at tilføje FastComments til dit nye site, se [Installationsguide](/guide-installation.html).