The **FastComments**-blokken understøtter single sign-on, så dine Shopify-kunder kan kommentere som sig selv uden at oprette en separat FastComments-konto.

### Sådan fungerer det

Når en besøgende, der er logget ind i din butik, åbner en side med **FastComments**-blokken:

1. Blokken opdager Shopify `customer` object.
2. Den sender kundens navn og e-mail til FastComments via en underskrevet app-proxy-anmodning.
3. FastComments opretter eller matcher en bruger med nøgle `shopify-{customerId}`, så den samme kunde altid svarer til den samme FastComments-bruger på tværs af sessioner og geninstallationer.
4. Besøgendes navn vises på deres kommentarer. De bliver ikke bedt om at logge ind igen.

Hvis besøgende ikke er logget ind i butikken, falder blokken tilbage til anonym kommentering (eller FastComments' log ind-flow, afhængigt af din widget-konfiguration).

### Slå SSO fra

SSO er slået til som standard for enhver **FastComments**-blok. For at slå det fra for en bestemt blok:

1. Åbn Shopify-temaeditoren.
2. Åbn den skabelon, der indeholder blokken, og klik på blokken for at markere den.
3. Fjern markeringen af **SSO**.
4. Klik på **Gem**.

Slå SSO fra, hvis du ønsker, at kommentatorer skal vælge en separat identitet til samtalen. For eksempel en intern community-side, hvor personale kommenterer under et andet visningsnavn.

### Hvad FastComments modtager

SSO-payloadet, der sendes for hver kunde, indeholder:

- Et bruger-id afledt af Shopify-kunde-id'et (`shopify-{customerId}`).
- Kundens e-mail (bruges til at identificere brugeren; vises ikke offentligt).
- Kundens visningsnavn (bruges som navnet, der vises som kommentarens forfatter).

Ingen ordrehistorik, betalings- eller adresseoplysninger sendes. Payloaden er underskrevet på serversiden; kundens browser ser aldrig en legitimationsoplysning.

### Log ind- og log ud-links

Når SSO er slået til, peger kommentar-widgetens log ind- og log ud-links på `/account/login` og `/account/logout`, de standard Shopify-ruter for kundekonti. Der er intet at konfigurere. Linkene fungerer for enhver butik med kundekonti aktiveret.