If je je site van WordPress verhuist en FastComments op de nieuwe site wilt, heb je de WordPress‑plug‑in niet nodig. Exporteer je reacties vanuit WordPress en upload vervolgens het bestand op de [Importpagina](https://fastcomments.com/auth/my-account/manage-data/import) in het FastComments‑dashboard.

We ondersteunen twee WordPress‑exportformaten.

### WordPress XML (Aanbevolen)

Dit is het bestand van de ingebouwde exporteur van WordPress, dus er is geen extra plug‑in nodig.

1. Ga in je WordPress‑admin naar `Tools -> Export`.
2. Selecteer `All content` en klik op `Download Export File`.
3. Op de FastComments [Importpagina](https://fastcomments.com/auth/my-account/manage-data/import) selecteer je `WordPress (.xml)` en upload je het bestand.

Elke reactie is gekoppeld aan de URL van het bericht waarop deze is geplaatst, die al in het bestand staat.

De import behoudt de naam, e‑mail en website van de auteur, de datum, de inhoud, de reply‑structuur en of de reactie is goedgekeurd. Reactie‑avatars worden overgenomen van Gravatar. Stemmen maken geen deel uit van dit formaat.

### WordPress CSV

Dit is het bestand van de [WordPress Comments Import & Export plug‑in van WebToffee](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Installeer de plug‑in in je WordPress‑admin en exporteer je reacties als CSV.
2. Vervang elke `comment_post_ID`‑waarde door de URL van het bericht.
3. Op de FastComments [Importpagina](https://fastcomments.com/auth/my-account/manage-data/import) selecteer je `WordPress (.csv)` en upload je het bestand.

Elke reactie is gekoppeld aan de kolom `comment_post_ID`. WordPress vult deze kolom met de bericht‑ID, en je nieuwe site heeft geen WordPress‑bericht‑ID's, dus stap 2 vervangt deze door de URL.

De import behoudt de naam, e‑mail en website van de auteur, de datum, de inhoud, de reply‑structuur en of de reactie is goedgekeurd. Reactie‑avatars worden overgenomen van Gravatar. Het behoudt ook de spam‑vlag van WordPress en de likes en dislikes van wpDiscuz wanneer het bestand deze bevat.

### Reacties koppelen aan je nieuwe pagina's

Als je nieuwe site dezelfde URL's behoudt als je WordPress‑site, verschijnen de reacties op de overeenkomende pagina's zonder extra configuratie.

Als het domein verandert, voer je na de import de [Domein‑migratietool](/guide-migrations.html#migrating-domains) uit. Als individuele pagin URL's veranderen, kun je [elke pagina migreren](/guide-migrations.html#migrating-pages) van de oude URL naar de nieuwe.

Voor bulk‑paginamigraties, zoals het verwijderen van het domein uit de waarde die je doorgeeft aan het [urlId](/guide-customizations-and-configuration.html#url-id) veld van de reactie‑widget, [open een supportticket](https://fastcomments.com/auth/my-account/help) en wij regelen het voor je.

### Voordat je overschakelt

Je kunt de import zo vaak uitvoeren als je wilt. Het opnieuw importeren van hetzelfde bestand [maakt geen duplicaten](/guide-migrations.html#importing-data), dus je kunt één keer importeren om de nieuwe site te testen, en vervolgens opnieuw importeren met je nieuwste reacties vlak voor de overstap.

Voor exportbestanden groter dan 1 GB, [neem contact op met support](https://fastcomments.com/auth/my-account/help).

Om FastComments aan je nieuwe site toe te voegen, zie de [Installatie‑gids](/guide-installation.html).