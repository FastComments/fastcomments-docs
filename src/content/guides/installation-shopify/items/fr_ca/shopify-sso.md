Le **FastComments** block prend en charge l'authentification unique (SSO) afin que vos clients Shopify puissent commenter en leur nom sans créer de compte FastComments distinct.

### Comment ça fonctionne

When a visitor who is logged into your store opens a page with the **FastComments** block:

1. The block detects the Shopify `customer` object.
2. It sends the customer's name and email to FastComments through a signed app proxy request.
3. FastComments creates or matches a user keyed by `shopify-{customerId}`, so the same customer always maps to the same FastComments user across sessions and re-installs.
4. The visitor's name shows up on their comments. They are not prompted to log in again.

If the visitor is not logged into the store, the block falls back to anonymous commenting (or the FastComments sign-in flow, depending on your widget configuration).

### Désactiver SSO

SSO is on by default for every **FastComments** block. To turn it off on a specific block:

1. Open the Shopify theme editor.
2. Open the template that contains the block and click the block to select it.
3. Uncheck **SSO**.
4. Click **Save**.

Désactivez SSO si vous voulez que les commentateurs choisissent une identité distincte pour la conversation. Par exemple, une page communautaire interne où le personnel commente sous un autre nom d'affichage.

### Ce que FastComments reçoit

The SSO payload sent for each customer contains:

- A user ID derived from the Shopify customer ID (`shopify-{customerId}`).
- The customer's email (used to identify the user; not displayed publicly).
- The customer's display name (used as their comment author name).

No order history, payment, or address data is sent. The payload is signed server-side; the customer's browser never sees a credential.

### Liens de connexion et de déconnexion

When SSO is on, the comment widget's sign-in and sign-out links point at `/account/login` and `/account/logout`, the standard Shopify customer account routes. There is nothing to configure. The links work for any store with customer accounts enabled.