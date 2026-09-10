---
Volg dezelfde stappen voor `localhost` als voor productie. Zorg ervoor dat je productiedomeinen en API‑geheimen hebt ingesteld.

Ga eerst naar de [Webhooks-beheer](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dit is toegankelijk via Beheer Data -> Webhooks.

De pagina geeft een overzicht van alle webhooks in je account:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks-beheerpagina die elke webhook weergeeft met zijn URL, gebeurtenis, domein, methode, status en aantal in de wachtrij staande gebeurtenissen'; title='Webhooks Lijst'; cacheBuster = 'v4' app-screenshot-end]

Klik op **Nieuwe Webhook** om er een toe te voegen. Elke webhook heeft een URL, één reactie‑gebeurtenis (aangemaakt, bijgewerkt of verwijderd), een domein en een HTTP‑methode:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Nieuw webhook‑formulier met velden voor URL, gebeurtenis, domein en HTTP‑methode plus knop Testpayload Verzenden'; title='Nieuwe Webhook'; cacheBuster = 'v4' app-screenshot-end]

Elke webhook wordt onafhankelijk afgeleverd. Je kunt dezelfde gebeurtenis naar meerdere eindpunten sturen, en een webhook die is ingesteld op **Alle Domeinen** ontvangt reacties van elk domein, zelfs wanneer er een domeinspecifieke webhook bestaat voor dezelfde gebeurtenis. Dezelfde URL, gebeurtenis en domein kunnen niet twee keer worden toegevoegd.

Klik vóór het opslaan op **Testpayload Verzenden** om te controleren of het eindpunt een ondertekend verzoek accepteert. Zie de volgende sectie, "Testen", voor details.

Vanaf de lijst kun je een webhook bewerken, uitschakelen, opnieuw inschakelen of verwijderen. Uitschakelen houdt de in de wachtrij staande gebeurtenissen vast totdat de webhook opnieuw wordt ingeschakeld; verwijderen gooit ze weg.

Webhooks kunnen ook via de API worden aangemaakt, bijvoorbeeld door Zapier. Deze verschijnen in dezelfde lijst met de bron **API**. Zie Webhooks beheren via de API.

---