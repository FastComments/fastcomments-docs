VanillaJS-versionen af widgetten er meget nem at installere og kræver ikke build-systemer eller server-side kode.

Tilføj blot følgende kodestykke til enhver side:

[code-example-start config = {}; title = 'Simpelt kodestykke'; code-example-end]

Du kan bruge det samme kodestykke på mange sider; det vil automatisk oprette en separat tråd for hver side.

Mange applikationer har en "HTML Embed Code" mulighed. Vælg det og indsæt kodestykket ovenfor.

*Du behøver ikke engang en konto for at prøve det!* Du kan se "tenantId: demo" i kodestykket ovenfor, hvis du ikke er logget ind. På den måde vil det bruge demo-kontoen.

Du kan finde dokumentation om konfiguration af widgetten <a href="/guide-customizations-and-configuration.html" target="_blank">her</a>.

Alle versioner af FastComments-widgetten er wrappers omkring det centrale VanillaJS-bibliotek. Dette giver os mulighed for at tilføje funktioner og rette problemer ét sted - og ændringerne spredes automatisk til de andre varianter af kommentar-widgetten.
