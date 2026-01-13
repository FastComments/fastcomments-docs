### Understøttelse af mørk tilstand

Image Chat inkluderer indbygget understøttelse af mørk tilstand. Når du sætter `hasDarkBackground: true` i din konfiguration, justerer chatvinduerne og UI-elementerne sig automatisk, så de fungerer godt på mørke baggrunde.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Stilen for mørk tilstand gælder for chatvinduer, markeringsfirkanter og alle interaktive elementer. Hvis dit site har en skiftefunktion for mørk tilstand, kan du geninitialisere widget'en, når tilstanden ændres, eller bruge body-klasse-tilgangen beskrevet nedenfor.

### Dynamisk mørk tilstand

Hvis dit sites mørke tilstand styres ved at tilføje en `.dark` klasse til body-elementet, vil Image Chat UI automatisk respektere dette uden at kræve geninitialisering. Widget'ens styles er designet til at reagere på tilstedeværelsen af denne klasse.

```css
/* Din CSS til mørk tilstand */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Tilpasset styling med CSS

Du kan tilpasse udseendet af markører, chatvinduer og andre elementer ved hjælp af CSS. Widget'en tilføjer specifikke klasser, som du kan målrette i dit stylesheet.

Chat-firkanterne og vinduerne bruger FastComments' kommentarboble-styling, så eventuelle tilpasninger, du har anvendt på den standard kommenteringswidget, også vil påvirke Image Chat.

### Størrelse på chat-firkanter

Optionen `chatSquarePercentage` kontrollerer størrelsen på de klikbare markører. Standardværdien er 5% af billedets bredde:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Større, mere synlige firkanter
});
```

Mindre værdier skaber mere diskrete markører, der blander sig med billedet. Større værdier gør markørerne mere fremtrædende og lettere at klikke på, især på mobile enheder eller med henblik på tilgængelighed.

### Mobiladfærd

På skærme under 768px i bredden skifter Image Chat automatisk til et mobiloptimeret layout. Chatvinduer vises i fuld skærm i stedet for at flyde ved siden af markørerne, hvilket giver bedre brugervenlighed på små skærme.

Markørerne forbliver synlige på deres responsive positioner på billedet. Brugere kan trykke på en vilkårlig markør for at åbne den fulde chatgrænseflade. Denne adfærd er indbygget og kræver ingen konfiguration.

### Udseende af chatvinduer

Chatvinduer er 300px brede på desktop med en 16px pil, der peger på markøren. Vinduerne positionerer sig automatisk baseret på tilgængelig plads i viewporten ved hjælp af positioneringsklasser som `to-right`, `to-left`, `to-top` og `to-bottom`.

Du kan tilføje brugerdefineret CSS for at justere farver, skrifttyper, afstande eller andre visuelle egenskaber af disse vinduer. Chatvinduerne bruger samme komponentstruktur som den standard FastComments-widget, så de arver alle globale tilpasninger, du har anvendt.

### Forsinket initialisering

Chatvinduer initialiseres ved hover for desktop-brugere eller øjeblikkeligt, når de oprettes. Dette reducerer den indledende load-overhead ved kun at rendere chatgrænsefladen, når brugere rent faktisk interagerer med en markør.

Den forsinkede initialisering sker transparent. Brugerne bemærker ikke nogen forsinkelse, men browseren behøver ikke at rende snesevis af skjulte chatvinduer, hvis du har mange markører på et billede.

### Lokalisering

Image Chat understøtter alle de samme lokaliseringsmuligheder som den standard FastComments-widget. Sæt optionen `locale` for at vise UI-tekst på forskellige sprog:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Fransk
});
```

FastComments understøtter dusinvis af sprog. Locale-indstillingen påvirker al UI-tekst inklusive prompts, knapper og pladsholdertekst.

### Arvede tilpasningsmuligheder

Da Image Chat udvider den standard kommenteringswidget, arver den alle tilpasningsmuligheder fra basiswidget'en. Dette inkluderer brugerdefinerede CSS-klasser, tilpassede oversættelser, avatartilpasning, datoformattering og meget mere.

Se hoveddokumentationen for FastComments-tilpasning for den komplette liste over tilpasningsmuligheder, der er tilgængelige.

### Arbejde med tilpassede skrifttyper

Hvis dit site bruger tilpassede skrifttyper, arver Image Chat UI disse skrifttyper fra din sides CSS. Chatvinduerne renderes inde i din sides DOM og respekterer dine eksisterende typografiske indstillinger.

For bedste resultat, sørg for at dine tilpassede skrifttyper er indlæst, før du initialiserer Image Chat, eller accepter at der kan forekomme et kort øjeblik med ustylet tekst, mens skrifttyperne indlæses.

### Visuelt design af markører

De firkantede markører har et subtilt visuelt design, der gør dem bemærkelsesværdige uden at overvælde billedet. Du kan tilpasse deres udseende med CSS, hvis du ønsker en anden visuel behandling.

Markørerne inkluderer hover-tilstande, der giver feedback, når brugere bevæger musen over dem. På touch-enheder giver trykinteraktionen øjeblikkelig feedback ved at åbne chatvinduet.

---