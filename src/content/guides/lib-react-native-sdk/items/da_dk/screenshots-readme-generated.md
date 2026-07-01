Live trådet kommentering med avatarer, indlejrede svar, stemmer og den indbyggede rich‑text‑composer, plus et mørkt tema og en live‑chat‑forudindstilling (vist her gengivet via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live‑kommentering</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live‑kommentering, lyst tema"/></td>
    <td align="center"><b>Mørkt tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live‑kommentering, mørkt tema"/></td>
    <td align="center"><b>Live‑chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live‑chat‑forudindstilling"/></td>
  </tr>
</table>

### Rich Text Editor

Dette bibliotek bruger [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) til rich‑text‑redigering, som giver en kraftfuld WYSIWYG‑redigeringsoplevelse. Den samme editor driver iOS, Android og web (via `react-native-web`), så composeren opfører sig konsistent på alle platforme med en enkelt implementering.

`react-native-enriched` kræver React Native New Architecture (Fabric) på native (standard siden RN 0.76, valgfrivær på RN 0.72‑0.75), samt en bundler der løser pakke‑`exports`‑betingelser. Dette SDK er udviklet og testet mod RN 0.81 / React 19. Den samme editor kører også på web gennem `react-native-web`; den berigede editors web‑build er fortsat markeret som eksperimentel upstream.

### Widgets

SDK'et leverer tre widgets, som spejler FastComments Android SDK:

- `FastCommentsLiveCommenting` – trådet kommentering med stemmer, svar, paginering, nævnelser, meddelelser og live‑opdateringer.
- `FastCommentsLiveChat` – en chat‑forudindstilling på samme motor: kronologiske beskeder med de nye i bunden, composeren under listen, en live‑header‑strip (forbindelsesprik + brugerantal), uendelig historik indlæst ved op‑scroll, auto‑scroll til nye beskeder, ingen stemmer eller svar‑trådning. Hver forudindstilling kan tilsidesættes via `config`.
- `FastCommentsFeed` – en social feed med post‑composer, medier, reaktioner, følgninger og live‑ny‑post‑bannere.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Standardudseendet genereres ud fra et sæt semantiske design‑tokens (`FastCommentsTheme`): farver, mellemrum, radius, skriftstørrelser, skrifttykkelser og avatar‑størrelser. Send delvise token‑overrides (type `FastCommentsThemeOverrides`) via `theme`‑prop'en på enhver widget, så omstillet hele stiltræet konsistent:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Mørkt tilstand er kun et token‑sæt væk:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles`‑prop'en accepterer stadig et rå `IFastCommentsStyles`‑træ for præcis kontrol. Når både `theme` og `styles` leveres, vinder de eksplicitte stilarter over det tematiserede træ; når kun `styles` leveres, erstatter det standarderne fuldstændigt (den oprindelige adfærd, så eksisterende integrationer og skins er upåvirkede). `setupDarkModeSkin` er udfaset til fordel for `theme`‑prop'en.

### Configuration Options

Dette bibliotek har til formål at understøtte alle konfigurationsmuligheder defineret i [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), præcis som web‑implementeringen.

Ud over disse tilføjer React Native et par SDK‑specifikke muligheder via `FastCommentsRNConfig`:

- `hideTopBar` – skjuler den loggede‑ind‑bruger / notifikations‑klokke‑strip, der vises over composeren.
- `usePressToEdit` – tryk‑og‑hold på en kommentar for at åbne dens menu.
- `disableDownVoting` – skjuler ned‑stem‑knapper.
- `renderCommentInline` – renderer kommentatorens info inden for samme HTML‑blok som kommentarens indhold.
- `renderLikesToRight` – flytter stemme/like‑området til højre for kommentaren i stedet for under den.
- `renderDateBelowComment` – renderer datoen under kommentaren.
- `showLiveStatus` – viser chat‑stil “Live” + bruger‑tælling header‑strip over kommentarer.
- `useInlineSubmitButton` – renderer send‑knappen som et ikon inde i composeren.
- `countAboveToggle` – sammen med `useShowCommentsToggle`, hvor mange kommentarer der renderes over “Vis kommentarer”‑toggles.
- `preserveFeedScrollPosition` – `FastCommentsFeed` husker sin scroll‑offset på tværs af unmount/remount (standard true).

### FastComments Concepts

De vigtigste koncepter at kende for at komme i gang er `tenantId` og `urlId`. `tenantId` er din FastComments.com‑kontoidentifikator. `urlId` er den, som kommentartråde knyttes til. Dette kan være en side‑URL, et produkt‑ID, et artikel‑ID osv.

### Localization

Al bruger‑vendt tekst i disse widgets (knap‑etiketter, pladsholdere, tom‑tilstande, relative datoer såsom “5 minutter siden”, fejmeddelelser osv.) er **server‑drevet**. Komponenterne hard‑coder ikke engelske strenge; de viser de oversættelser, FastComments leverer for den anmodede locale.

For at anmode om en locale, sæt `locale` i din konfiguration:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Når ingen `locale` er sat, leverer FastComments lejerens standardsprog.

**Redigering af teksten:** oversættelser håndteres i dit FastComments‑dashboard, ikke i dette SDK. For at ændre formuleringen, tilsidesæt standardteksten, eller tilføj et sprog, rediger oversættelserne for din konto i dashboardet – ændringen opfanges automatisk af widgets uden at kræve en app‑release. SDK'et leverer ingen engelske fallback‑tekster, så enhver nøgle du tømmer i dashboardet gengiver tomt; hold nøglerne udfyldt for hver locale du understøtter.

### User Notifications

FastComments understøtter notifikationer for [mange scenarier](https://docs.fastcomments.com/guide-notifications.html). Notifikationer er konfigurerbare, kan fravælges globalt eller på notifikations‑/kommentar‑niveau, og understøtter side‑niveau abonnementer, så brugere kan abonnere på tråde fra en specifik side eller artikel.

For eksempel er det muligt at bruge Secure SSO til at autentificere brugeren og derefter periodisk poll for ulæste notifikationer og push dem til brugeren.

Se [eksempel‑appen AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for hvordan man henter og oversætter ulæste brugernotifikationer.

### Gif Browser

Som standard er ingen billed‑ eller gif‑valg aktiveret. Se [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for hvordan du understøtter billede‑ og gif‑uploads. Der er en Gif‑browser, der anonymiserer søgninger og billeder leveret i dette bibliotek; du skal blot bruge den.

### Performance

Åbn venligst en sag med et eksempel til reproduktion, inklusiv den anvendte enhed, hvis du opdager performance‑problemer. Performance er en førsteklasses funktion i alle FastComments‑biblioteker.