## Voorbeeld Zaps

Een paar workflows die in enkele minuten zijn opgezet.

**Ontvang een melding over nieuwe reacties.** Nieuwe reactie, dan Slack "Send Channel Message" of Discord "Send Channel Message". Koppel de naam van de reageerder, de reactietekst en de pagin URL in het bericht. Voeg de domeinfilter toe om een ander kanaal per site te informeren.

**Houd een log bij van elke reactie.** Nieuwe reactie, dan Google Sheets "Create Spreadsheet Row". Voeg Deleted Comment toe als een tweede Zap die een rij toevoegt met de reactie‑id, zodat het blad tevens dient als audit‑trail.

**E‑mail de auteur wanneer een reactie is goedgekeurd.** Updated Comment met een Zapier‑filter op Approved is true, dan Gmail "Send Email". Omdat Updated Comment bij elke wijziging wordt geactiveerd, zorgt het filter ervoor dat deze Zap alleen reageert op goedkeuringen.

**Voeg reageerders toe aan je CRM of mailinglijst.** Nieuwe reactie, dan HubSpot "Create or Update Contact" of Mailchimp "Add or Update Subscriber" met het e‑mailadres van de reageerder. Houd rekening met je privacybeleid en lokale wetgeving voordat je iemand aan een marketinglijst toevoegt.

**Maak een reactie vanuit een formulier.** Typeform of Google Forms "New Response", dan FastComments Create Comment met de pagina‑URL‑ID die je site gebruikt voor testimonials. Laat Approved uitgevinkt zodat je elke reactie kunt beoordelen voordat deze verschijnt.

**Plaats aankondigingen in een feed.** RSS by Zapier "New Item in Feed", dan Create Feed Post met de titel, inhoud en link van het item.

**Voorzie leden van SSO‑gebruikers.** Memberstack, Memberful, of je eigen webhook, dan Find SSO User gevolgd door Create SSO User in "find or create"‑modus.

**Escaleer gerapporteerde reacties.** Updated Comment, gefilterd op een vlag‑aantal hoger dan nul, dan Trello "Create Card" of Linear "Create Issue" met de reactie‑id en een link naar de moderatiepagina.

**Publiceer pagina's zodra ze live gaan.** WordPress of Ghost "New Post", dan Create Page met de post‑URL, zodat de pagina wordt vermeld en beperkt voordat de eerste reactie verschijnt.

**Archiveer verwijderde reacties.** Deleted Comment, dan Airtable "Create Record" met de volledige reactie voor nalevingsbewaring.