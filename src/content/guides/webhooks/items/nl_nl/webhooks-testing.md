De nieuwe en bewerk webhook‑pagina's hebben een `Send Test Payload`‑knop die een verzoek naar de URL in het formulier stuurt, ongeacht of deze is opgeslagen. De Create‑ en Update‑gebeurtenissen sturen een dummy WebhookComment‑object, terwijl bij het testen van Delete een dummy request‑body met alleen een ID wordt verzonden.

## Verifiëren van payloads

Bij het testen van je webhook‑integratie, controleer of de binnenkomende verzoeken de volgende headers bevatten:

1. **`X-FastComments-Timestamp`** – Unix‑tijdstempel (seconden)  
2. **`X-FastComments-Signature`** – HMAC‑SHA256‑handtekening  

Webhooks die zijn aangemaakt vóór de introductie van het handtekeningschema ontvangen ook een **`token`**‑header met je API‑secret. Nieuwe webhooks niet.

Gebruik de HMAC‑handtekeningverificatie om te garanderen dat payloads authentiek zijn.

## Testhulpmiddelen

Je kunt tools zoals [webhook.site](https://webhook.site) of [ngrok](https://ngrok.com) gebruiken om binnenkomende webhook‑payloads tijdens de ontwikkeling te inspecteren.

## Evenementtypen

- **Create Event**: Wordt geactiveerd wanneer een nieuw commentaar wordt aangemaakt.  
- **Update Event**: Wordt geactiveerd wanneer een commentaar wordt bewerkt.  
- **Delete Event**: Wordt geactiveerd wanneer een commentaar wordt verwijderd.  

Elke webhook is gekoppeld aan één evenement en één HTTP‑methode (POST, PUT of DELETE). Elk evenement bevat de volledige commentaargegevens in de request‑body (zie [Data Structures](/guide-webhooks.html#webhooks-structures) voor het payload‑formaat).