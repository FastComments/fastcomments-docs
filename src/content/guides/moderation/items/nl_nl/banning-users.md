Er zijn twee manieren om gebruikers te blokkeren van het plaatsen van reacties op uw site met FastComments.

De eerste is als u hun e-mailadres al kent, kunt u het invoeren op de <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">pagina met geblokkeerde gebruikers</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lijst met geblokkeerde gebruikers onder Modereren van reacties, met de geblokkeerde e‑mailadressen en een knop om een nieuwe blokkering toe te voegen'; title='De pagina met geblokkeerde gebruikers' app-screenshot-end]

Deze pagina is toegankelijk via Modereren van reacties -> Geblokkeerde gebruikers

Wanneer we een gebruiker willen blokkeren, kunnen we een type kiezen, ofwel Permanent of Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Nieuw blokkeringformulier met een e‑mailveld en een keuzemogelijkheid voor het type blokkering: Permanent of Permanent Shadow Ban'; title='Een gebruiker blokkeren' app-screenshot-end]

De tweede manier om een gebruiker te blokkeren is door op de blokkeerknop te klikken die op elk commentaar staat op de pagina Commentaarmoderatie.

Wanneer we op de blokkeerknop klikken, krijgt u enkele opties te zien, waarin we het type blokkering en de duur kunnen specificeren.

### E-mailaliassen

Bij het blokkeren van een gebruiker via e‑mail negeert FastComments automatisch `+`‑aliassen. Bijvoorbeeld, het blokkeren van `user+alias@gmail.com` zal ook `user@gmail.com` en elke andere `+`‑variatie van dat adres blokkeren, zoals `user+other@gmail.com`.

### Shadow-blokkeringen

Een shadow-ban is een type blokkering waarbij het lijkt alsof het commentaar of de stem van de gebruiker succesvol is opgeslagen, terwijl dat in werkelijkheid niet het geval is. Dit kan in bepaalde situaties wenselijk zijn.

### Blokkeren via IP-adres

Tenzij een huurder ervoor kiest zich af te melden, ondersteunt FastComments het blokkeren via IP door een gehashte versie van het IP‑adres van de commentator op te slaan.