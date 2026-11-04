Wanneer gebruikers reageren of stemmen, en ze zijn niet ingelogd, wordt hen gevraagd hun e-mailadres en gebruikersnaam op te geven.

Voor sommige sites is het vragen aan een bezoeker om een unieke gebruikersnaam te verzinnen een drempel, vooral op mobiel. FastComments kan
een neutrale gebruikersnaam genereren voor elke nieuwe bezoeker en deze vooraf invullen in het veld gebruikersnaam, zoals `BraveOtter4172`.

De bezoeker kan deze laten staan, of vervangen door een naam naar eigen keuze.

Dit kan worden ingeschakeld via de Customization UI, onder de instelling genaamd `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='De optie Generate Usernames Automatically in de widget customization UI'; title='Generate Usernames Automatically' app-screenshot-end]

#### Hoe het werkt

- Elke gegenereerde naam is uniek. Deze wordt gecontroleerd tegen bestaande accounts en gereserveerd voor de browsersessie van die bezoeker, zodat twee bezoekers niet dezelfde naam aangeboden krijgen.
- De naam wordt alleen gegenereerd voor bezoekers die er nog geen hebben. Ingelogde gebruikers, SSO-gebruikers en bezoekers die al hebben gereageerd behouden hun bestaande naam.
- Het werkt met of zonder [anoniem reageren](/guide-customizations-and-configuration.html#allow-anon). Met anoniem reageren uitge schakeld vult de bezoeker nog steeds zijn e-mailadres in, maar hoeft niet langer een gebruikersnaam te bedenken.
- Een terugkerende bezoeker die een e-mailadres invult dat hij eerder heeft gebruikt, wordt gekoppeld aan zijn bestaande account en behoudt de naam van dat account.
- Als ook een [Default Username](/guide-customizations-and-configuration.html#default-username) is ingesteld, heeft de gegenereerde naam voorrang.