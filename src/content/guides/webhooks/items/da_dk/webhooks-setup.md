---
Følg de samme trin for `localhost`, som du ville gøre for produktion. Sørg for, at du har produktionsdomæner og API-hemmeligheder konfigureret.

Først, naviger til [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dette er tilgængeligt via Manage Data -> Webhooks.

Siden viser alle webhooks på din konto:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks-adminside, der viser hver webhook med dens URL, hændelse, domæne, metode, status og antallet af køede hændelser'; title='Webhooks List'; cacheBuster = 'v4' app-screenshot-end]

Klik på **New Webhook** for at tilføje en. Hver webhook har en URL, én kommentarhændelse (oprettet, opdateret eller slettet), et domæne og en HTTP-metode:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Ny webhook-formular med felterne URL, hændelse, domæne og HTTP-metode samt Send Test Payload'; title='New Webhook'; cacheBuster = 'v4' app-screenshot-end]

Hver webhook leveres uafhængigt. Du kan sende den samme hændelse til flere endpoints, og en webhook med omfang **All Domains** modtager kommentarer fra alle domæner, selv når der findes en domænespecifik webhook for den samme hændelse. Den samme URL, hændelse og domæne kan ikke tilføjes to gange.

Før du gemmer, klik på **Send Test Payload** for at kontrollere, at endpointet accepterer en signeret anmodning. Se næste afsnit, "Testing", for detaljer.

Fra listen kan du redigere, deaktivere, genaktivere eller slette en webhook. Deaktivering bevarer køede hændelser, indtil webhooken genaktiveres; sletning kasserer dem.

Webhooks kan også oprettes via API'en, for eksempel af Zapier. Disse vises i den samme liste med kilden **API**. Se Managing Webhooks via the API.

---