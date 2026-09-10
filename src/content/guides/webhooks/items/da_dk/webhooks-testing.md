De nye og redigerings‑webhook‑sider har en `Send Test Payload`‑knap, der sender en anmodning til den URL, der i øjeblikket er i formularen, uanset om den er gemt eller ej. Create‑ og Update‑begivenhederne sender et dummy‑WebhookComment‑objekt, mens test af Delete vil sende et dummy‑anmodnings‑body med kun et ID.

## Verificering af payloads

Når du tester din webhook‑integration, skal du verificere, at de indgående anmodninger indeholder følgende headers:

1. **`X-FastComments-Timestamp`** - Unix‑tidsstempel (sekunder)  
2. **`X-FastComments-Signature`** - HMAC‑SHA256‑signatur  

Webhooks oprettet før signaturskemaet blev introduceret modtager også en **`token`**‑header, der indeholder din API‑hemmelighed. Nye webhooks gør det ikke.

Brug HMAC‑signaturverificering for at sikre, at payloads er ægte.

## Testværktøjer

Du kan bruge værktøjer som [webhook.site](https://webhook.site) eller [ngrok](https://ngrok.com) til at inspicere indgående webhook‑payloads under udvikling.

## Begivenhedstyper

- **Create Event**: Udløses, når en ny kommentar oprettes.  
- **Update Event**: Udløses, når en kommentar redigeres.  
- **Delete Event**: Udløses, når en kommentar slettes.  

Hver webhook er knyttet til én begivenhed og én HTTP‑metode (POST, PUT eller DELETE). Hver begivenhed inkluderer de fulde kommentardata i anmodnings‑body’en (se [Data Structures](/guide-webhooks.html#webhooks-structures) for payload‑formatet).