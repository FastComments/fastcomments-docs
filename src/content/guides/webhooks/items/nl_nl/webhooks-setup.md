---
Volg dezelfde stappen voor `localhost` als voor productie. Zorg ervoor dat je productiedomeinen en API‑geheimen hebt ingesteld.

Ga eerst naar de [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Deze is toegankelijk via Beheer gegevens -> Webhooks.

De configuratiepagina verschijnt als volgt:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks-beheerpagina met een domeinselector en een eindpunt‑URL‑veld per reactie‑gebeurtenis, plus Send Test Payload'; title='Webhooks-configuratie'; cacheBuster = 'v3' app-screenshot-end]

Op deze pagina kun je eindpunten opgeven voor elk type reactie‑gebeurtenis.

Voor elk type gebeurtenis moet je op 'Send Test Payload' klikken om er zeker van te zijn dat je integratie correct is ingesteld. Zie de volgende sectie, "Testing", voor details.

---