Er zijn twee manieren om gebruikers te verbieden te reageren op uw site met FastComments.

De eerste is als u hun e‑mail al kent, kunt u deze invoeren op de <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">verboden gebruikers</a> pagina.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lijst met verboden gebruikers onder Modereren van reacties, met de verboden e‑mailadressen en een knop om een nieuw verbod toe te voegen'; title='De pagina Verboden gebruikers' app-screenshot-end]

Deze pagina is toegankelijk via Modereren van reacties -> Verboden gebruikers

Wanneer we een gebruiker willen verbieden, kunnen we een type kiezen, ofwel Permanent of Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Nieuw verbodformulier met een e‑mailveld en een keuzemogelijkheid voor verbodtype Permanent of Permanent Shadow Ban'; title='Een gebruiker verbieden' app-screenshot-end]

De tweede manier om een gebruiker te verbieden is door op de verbodknop te klikken die op elk commentaar op de pagina Commentaarmoderatie staat.

Wanneer we op de verbodknop klikken, krijgt u enkele opties te zien, waarin we het verbodtype en de duur kunnen specificeren.

### E‑mailaliassen

Bij het verbieden van een gebruiker via e‑mail negeert FastComments automatisch `+`‑aliassen. Bijvoorbeeld, het verbieden van `user+alias@gmail.com` zal ook `user@gmail.com` en elke andere `+`‑variant van dat adres verbieden, zoals `user+other@gmail.com`.

### Shadow‑verboden

Een shadow‑ban is een type verbod dat doet lijken alsof het commentaar of de stem van de gebruiker succesvol is opgeslagen, terwijl dat in werkelijkheid niet het geval is. Dit kan in bepaalde situaties wenselijk zijn.

### Verbieden via IP‑adres

Tenzij een huurder ervoor kiest zich af te melden, ondersteunt FastComments het verbieden via IP door een gehashte versie van het IP‑adres van de commentator op te slaan.

### Verboden gebruikers zoeken

Wanneer uw lijst groter wordt dan een of twee pagina's, kunt u deze verfijnen met de zoekbalk boven de tabel.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Zoekbalk op de pagina Verboden gebruikers met een vervolgkeuzelijst Zoeken op, een vervolgkeuzelijst Overeenkomst en een invoerveld Waarde'; title='Verboden gebruikers zoeken' app-screenshot-end]

Er zijn drie bedieningselementen:

- **Search By** kiest in welk veld gezocht moet worden: Any Field, Email, Name, Banned By, of Banned For Saying. De laatste vier komen overeen met de kolommen met dezelfde naam in de tabel.
- **Match** kiest hoe er vergeleken wordt. **Contains** vindt uw waarde overal in het veld, en **Equals** komt overeen met het volledige veld.
- **Value** is de tekst waarnaar gezocht wordt.

Elk veld wordt zonder onderscheid tussen hoofd‑ en kleine letters vergeleken, dus zoeken naar `SPAMMER@EXAMPLE.COM` vindt een verbod dat is opgeslagen als `spammer@example.com`.

Een paar zaken die het waard zijn om te weten:

- **Banned For Saying** zoekt in de tekst van het commentaar dat de gebruiker heeft laten verbieden. Zo vindt u iedereen die verboden is vanwege een bepaalde zin.
- **Banned By** zoekt de naam van de moderator die het verbod heeft opgelegd, wat nuttig is om de beslissingen van een andere moderator te beoordelen.
- Wildcard‑verboden worden opgeslagen met hun `*`, dus een **Contains**‑zoekopdracht naar `bademail.com` vindt een `*@bademail.com` verbod.
- **Name** komt overeen met de naam die in de kolom Naam wordt weergegeven, zodat het een gebruiker vindt zelfs als hij zijn naam heeft gewijzigd sinds het verbod, en zelfs als u het verbod hebt aangemaakt door een e‑mailadres in te voeren en er op dat moment geen naam werd vastgelegd. De op het verbod geregistreerde naam komt ook overeen, dus zoeken naar zowel de oude als de huidige naam werkt.
- **Any Field** zoekt tegelijk in het e‑mail, de naam, de moderator die het verbod heeft opgelegd, en de tekst van het verboden commentaar.

Uw zoekopdracht maakt deel uit van de pagin URL, zodat u een gefilterde lijst met andere moderators kunt delen op dezelfde manier als u andere moderatielinks deelt. Pagineren door de resultaten behoudt de zoekopdracht, een nieuwe zoekopdracht starten brengt u terug naar de eerste pagina, en **Clear** brengt u terug naar de volledige lijst.