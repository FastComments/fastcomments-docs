Følg de samme trin for `localhost`, som du ville for produktion. Sørg for, at du har opsat produktionsdomæner og API Secrets.

Først, gå til [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dette er tilgængeligt via Manage Data -> Webhooks.

Konfigurationssiden ser sådan ud:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

På denne side kan du angive endpoints for hver type kommentarhændelse.

For hver type hændelse skal du sørge for at klikke på Send Test Payload for at sikre, at du har opsat din integration korrekt. Se næste afsnit, "Testing", for detaljer.

---