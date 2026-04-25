En delmængde af moderationshandlinger kan udføres direkte fra kommentartråden selv, uden at man behøver gå til siden Kommentar-moderation.

Når du er logget ind, klik på redigeringsknappen øverst til højre i en kommentar. Som moderator bør du have følgende valgmuligheder:

- **Fastgør** den kommentar
- **Slet** den kommentar
- **Slet** den kommentar + **Forbyd brugeren** (Permanent eller Shadow, flere detaljer senere)
- **Rediger** den kommentar
- **Lås** eller **Oplås** den kommentar (flere detaljer nedenfor)
- Marker den kommentar som **Godkendt** (vis den) eller **Ikke godkendt** (skjul den)
- Marker den kommentar som **Spam** eller **Ikke spam**

### Låsning af en kommentar

Låsning af en individuel kommentar forhindrer nye svar på den og forhindrer også, at kommentaren selv bliver redigeret eller slettet, indtil den bliver oplåst. Dette gælder for alle, inklusive administratorer og moderatorer. Hvis du skal redigere eller fjerne en låst kommentar, oplås den først, foretag ændringen, og lås den igen, hvis ønsket.

Et hængelås-ikon vises i øverste højre hjørne af en låst kommentar, så læserne med det samme kan se, at tråden er lukket. Rediger- og Slet-menuindgangene skjules for låste kommentarer både i kommentarboksen og i det offentlige API (`PATCH` og `DELETE` returnerer `code: 'locked'` hvis de kaldes mod en låst kommentar).

To bevidste undtagelser omgår låsen, fordi de ellers ville efterlade forældreløse data: når en bruger sletter hele deres konto (deres kommentarer bliver ryddet op uanset låsens tilstand), og når en moderator forbyder en bruger med indstillingen "slet alle kommentarer fra denne bruger" (oprydningen fjerner låsene).

### Lukning af kommentartråde

Moderatorer og administratorer kan låse eller lukke kommentartråde ved at vælge `Close Thread` i tre-punktsmenuen øverst i kommentarfeltet, hvis de er logget ind. De kan senere vælge `Re-Open Thread` til enhver tid for at genåbne kommenteringen.

Lukning af en kommentartråd forhindrer nye kommentarer, men tillader stadig afstemning og brugere at slette deres kommentarer, hvis de ønsker det.

Lukning og genåbning af kommentartråde påvirker øjeblikkeligt alle brugere, der ser tråden.

Du kan også markere en tråd som skrivebeskyttet, hvilket også fjerner stemme- og sletningsmuligheder, ved at oprette en tilpasningsregel specifikt for den side.

### Opdateres i realtid

Alle disse handlinger vil opdatere kommentartrådene hos andre brugere med det samme, uden at de behøver genindlæse siden. Moderationshandlinger som at skjule en kommentar eller markere den som spam fjerner dog ikke kommentaren fra **moderatorens** skærm, så de hurtigt kan fortryde handlingen om nødvendigt. For at indikere, at en kommentar er skjult, vil den blive fremhævet i forhold til de andre kommentarer (fremhævelsesfarven afhænger af årsagen til fjernelsen).

For eksempel, givet brugerne `A (commenter)`, `B (Moderator 1)`, og `C (Moderator 2)`.

...og følgende scenarie:

1. `User B (Moderator 1)` skjuler en kommentar.
2. For `User A (commenter)` bliver den kommentar straks skjult.
3. For `User C (Moderator 2)` bliver den kommentar straks skjult.
4. For brugeren, som foretog ændringen, `User B (Moderator 1)`, forbliver kommentaren på deres skærm, men er fremhævet som fjernet. De har mulighed for at fortryde deres handling, i hvilket tilfælde de andre brugere igen vil se opdateringen i realtid.