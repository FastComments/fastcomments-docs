#### Naviger til Canvas LTI Config

Log ind på din FastComments-konto, og gå til <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Opret en ny LTI-konfiguration

Marker afkrydsningsfeltet **Enable LTI**. Konfigurationsfelterne vises:

- **Configuration Name** - en valgfri etiket til at identificere denne konfiguration (nyttigt hvis du forbinder flere Canvas-instanser).
- **Platform URL** - din Canvas-instances URL (f.eks. `https://yourschool.instructure.com`). Du kan lade dette stå tomt indtil videre og udfylde det efter at have oprettet Developer Key.
- **Client ID** - lad dette stå tomt for nu. Du udfylder det efter at have oprettet Developer Key i Canvas.
- **Deployment ID** - lad dette stå tomt for nu.
- **Comment Style** - vælg mellem Comments, Collab Chat eller Both. Se [Commenting Styles](#canvas-lms-commenting-styles) for detaljer.

Klik på **Add** for at oprette konfigurationen.

#### Kopiér Configuration URLs

Efter gemning vises der tre URL'er:

- **Configuration URL** - du indsætter denne i Canvas, når du opretter Developer Key.
- **OIDC Login URL** - bruges af Canvas til LTI-loginflowet (automatisk konfigureret via Configuration URL).
- **Launch URL** - endpointet, som Canvas kalder, når en studerende åbner FastComments (automatisk konfigureret via Configuration URL).

Kopier **Configuration URL**. Du får brug for den i næste trin.