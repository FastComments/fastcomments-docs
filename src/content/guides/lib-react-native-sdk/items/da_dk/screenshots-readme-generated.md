Live trådet kommentering med avatarer, indlejrede svar, stemmer og den indbyggede rich‑text‑komponist, plus et mørkt tema og en live‑chat‑forudindstilling (vist her gengivet via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Live-kommentering, lyst tema"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Live-kommentering, mørkt tema"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Live chat-forudindstilling"/></td>
  </tr>
</table>

### Rich Text Editor

Dette bibliotek bruger [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) til rich‑text‑redigering, som giver en kraftfuld WYSIWYG‑redigeringsoplevelse. Den samme editor driver iOS, Android og web (via `react-native-web`), så komponisten opfører sig konsistent på alle platforme med en enkelt implementering.

`react-native-enriched` kræver React Native New Architecture (Fabric) på native (standard siden RN 0.76, valgfrivær på RN 0.72‑0.75), og en bundler der løser pakke‑`exports`‑betingelser. Dette SDK er udviklet og testet mod RN 0.81 / React 19. Den samme editor kører også på web gennem `react-native-web`; den web‑bygning af enriched‑editoren er stadig markeret som eksperimentel upstream.

### Widgets

SDK’et leverer tre widgets, som spejler FastComments Android SDK:

- `FastCommentsLiveCommenting` – trådet kommentering med stemmer, svar, paginering, nævnelser, notifikationer og live‑opdateringer.
- `FastCommentsLiveChat` – en chat‑forudindstilling over den samme motor: kronologiske beskeder med nye nederst, komponist under listen, en live‑header‑stribe (forbindelses‑dot + bruger‑tæller), uendelig historik indlæst ved at scrolle op, auto‑scroll til nye beskeder, ingen stemmer eller svar‑trådning. Hver forudindstilling kan overskrives via `config`.
- `FastCommentsFeed` – et socialt feed med post‑komponist, medier, reaktioner, følgere og live‑ny‑post‑bannere.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Standardudseendet genereres ud fra et sæt semantiske design‑tokens (`FastCommentsTheme`): farver, afstande, radius, skrifttypestørrelser, skrifttype‑vægte og avatar‑størrelser. Send delvise token‑overrides (type `FastCommentsThemeOverrides`) gennem `theme`‑prop’en på enhver widget, så restyler hele stil‑træet konsistent:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Mørk tilstand er kun et token‑sæt væk:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles`‑prop’en accepterer stadig et råt `IFastCommentsStyles`‑træ for kirurgisk kontrol. Når både `theme` og `styles` leveres, vinder de eksplicitte styles over det tematiserede træ; når kun `styles` leveres, erstatter det standarderne fuldstændigt (den oprindelige opførsel, så eksisterende integrationer og skins påvirkes ikke). `setupDarkModeSkin` er udfaset til fordel for `theme`‑prop’en.

### Configuration Options

Dette bibliotek har til formål at understøtte alle konfigurationsmuligheder defineret i [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), ligesom web‑implementeringen.

Ud over dem tilføjer React Native nogle SDK‑specifikke muligheder via `FastCommentsRNConfig`:

- `hideTopBar` – skjul den loggede‑ind‑bruger / notifikations‑klokke‑stribe vist over komponisten.
- `usePressToEdit` – tryk‑og‑hold på en kommentar for at åbne dens menu.
- `disableDownVoting` – skjul ned‑stem‑knapper.
- `renderCommentInline` – render kommentator‑info inden for samme HTML‑blok som kommentar‑indholdet.
- `renderLikesToRight` – flyt stem‑/like‑området til højre for kommentaren i stedet for under den.
- `renderDateBelowComment` – render datoen under kommentaren.
- `showLiveStatus` – vis chat‑stil “Live” + bruger‑tæller‑header‑stribe over kommentarer.
- `useInlineSubmitButton` – render send‑knappen som et ikon inde i komponisten.
- `countAboveToggle` – med `useShowCommentsToggle`, hvor mange kommentarer der renderes over “Show Comments”‑toggle.
- `preserveFeedScrollPosition` – `FastCommentsFeed` husker sin scroll‑offset på tværs af unmount/remount (standard true).

### FastComments Concepts

De vigtigste begreber at kende for at komme i gang er `tenantId` og `urlId`. `tenantId` er din FastComments.com‑konto‑identifikator. `urlId` er hvor kommentartråde knyttes til. Dette kan være en side‑URL, et produkt‑ID, en artikel‑ID osv.

### Localization

Al bruger‑vendt tekst i disse widgets (knap‑etiketter, pladsholdere, tomme tilstande, relative datoer som “5 minutes ago”, fejlmeddelelser osv.) er **server‑drevet**. Komponenterne hard‑coder ikke engelske strenge; de viser de oversættelser FastComments leverer for den anmodede locale.

For at anmode om en locale, sæt `locale` i din konfiguration:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Når ingen `locale` er sat, leverer FastComments lejerens standardsprog.

**Redigering af teksten:** oversættelser håndteres i dit FastComments‑dashboard, ikke i dette SDK. For at ændre formulering, overskriv standard‑teksten, eller tilføj et sprog, rediger oversættelserne for din konto i dashboardet – ændringen opfanges automatisk af widgets uden at en app‑release er påkrævet. SDK’et leverer ingen engelske fallback‑tekster, så enhver nøgle du tømmer i dashboardet renderes tom; hold nøglerne udfyldt for hver locale du understøtter.

### User Notifications

FastComments understøtter notifikationer for [mange scenarier](https://docs.fastcomments.com/guide-notifications.html). Notifikationer er konfigurerbare, kan fravælges globalt eller på notifikations‑/kommentar‑niveau, og understøtter side‑niveau abonnementer så brugere kan abonnere på tråde fra en specifik side eller artikel.

For eksempel er det muligt at bruge Secure SSO til at autentificere brugeren og derefter periodisk poll’e for ulæste notifikationer og skubbe dem til brugeren.

Se [eksempel‑appen AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for hvordan man henter og viser ulæste bruger‑notifikationer.

### Gif Browser

Som standard er ingen billed‑ eller gif‑valg aktiveret. Se [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for hvordan du understøtter billede‑ og gif‑uploads. Der findes en Gif‑Browser som anonymiserer søgninger og billeder leveret i dette bibliotek; du skal blot bruge den.

### Performance

Åbn venligst en ticket med et eksempel til reproduktion, inklusiv den anvendte enhed, hvis du identificerer nogen præstationsproblemer. Performance er en førsteklasses borger i alle FastComments‑biblioteker.