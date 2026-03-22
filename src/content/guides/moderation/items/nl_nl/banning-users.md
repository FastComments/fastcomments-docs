Er zijn twee manieren om gebruikers te blokkeren zodat ze geen reacties meer kunnen plaatsen op uw site met FastComments.

De eerste is: als u hun e-mailadres al weet, kunt u het invoeren op de <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">banned users</a> pagina.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Deze pagina is toegankelijk via Reacties modereren -> Geblokkeerde gebruikers

Wanneer we een gebruiker willen verbannen, kunnen we een type kiezen: Permanent of permanente schaduwverbanning:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

De tweede manier om een gebruiker te verbannen is door op de verbodsknop te klikken die bij elke reactie op de pagina Reactie-moderatie staat.

Wanneer we op de verbodsknop klikken, krijgt u een aantal opties te zien, waarmee we het type verbanning en de duur kunnen opgeven.

### E-mailaliassen

Bij het verbannen van een gebruiker op e-mailadres negeert FastComments automatisch `+` aliassen. Bijvoorbeeld, het verbannen van `user+alias@gmail.com` zal ook `user@gmail.com` en elke andere `+`-variant van dat adres verbannen, zoals `user+other@gmail.com`.

### Schaduwverbanningen

Een schaduwverbanning is een type verbanning dat doet alsof de reactie of stem van de gebruiker succesvol is opgeslagen, terwijl dat in werkelijkheid niet het geval is. Dit kan in bepaalde situaties wenselijk zijn.

### Verbanning via IP-adres

Tenzij een tenant ervoor kiest zich af te melden, ondersteunt FastComments verbanningen via IP door een gehashte versie van het IP-adres van de reageerder op te slaan.