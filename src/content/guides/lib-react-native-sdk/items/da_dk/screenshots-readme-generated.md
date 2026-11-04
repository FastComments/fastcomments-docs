Live trådet kommentering med avatarer, indlejrede svar, stemmer og den indbyggede rich-text-editor, plus et mørkt tema og en live-chat-forindstilling (vist her renderet via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Live kommentering, lyst tema"/></td>
    <td align="center"><b>Mørkt tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Live kommentering, mørkt tema"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Live chat-forindstilling"/></td>
  </tr>
</table>

### Rich Text Editor

Dette bibliotek bruger [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) til rich text-redigering, hvilket giver en kraftfuld WYSIWYG-redigeringsoplevelse. Den samme editor driver iOS, Android og web (via `react-native-web`), så editoren opfører sig konsistent på tværs af alle platforme med en enkelt implementering.

`react-native-enriched` kræver React Native New Architecture (Fabric) på native (standard siden RN 0.76, opt-in på RN 0.72-0.75) og en bundler, der resolver package `exports`-betingelser. Denne SDK er udviklet og testet mod RN 0.81 / React 19. Den samme editor kører også på web gennem `react-native-web`; den berigede editors web-build er stadig markeret som eksperimentel upstream.

### Widgets

SDK'et leveres med tre widgets, der afspejler FastComments Android SDK:

- `FastCommentsLiveCommenting` - trådet kommentering med stemmer, svar, paginering, omtaler, notifikationer og live-opdateringer.
- `FastCommentsLiveChat` - en chat-forindstilling over den samme motor: kronologiske beskeder med nye nederst, editoren under listen, en live header-bjælke (forbindelsespunkt + brugerantal), uendelig historik indlæst ved at scrolle op, auto-scroll til nye beskeder, ingen stemmer eller svartrådning. Enhver forindstilling kan tilsidesættes via `config`.
- `FastCommentsFeed` - et socialt feed med post-editor, medier, reaktioner, følgere og live-bannere for nye opslag.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Temadesign

Standardudseendet genereres fra et sæt semantiske design-tokens (`FastCommentsTheme`): farver, afstande, radius, skriftstørrelser, skriftvægte og avatarstørrelser. Send delvise token-tilsidesættelser (typet `FastCommentsThemeOverrides`) gennem `theme`-prop'en på enhver widget, og hele stiltræet restyles konsistent:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Mørk tilstand er kun ét token-sæt væk:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles`-prop'en accepterer stadig et råt `IFastCommentsStyles`-træ for kirurgisk kontrol. Når både `theme` og `styles` er angivet, vinder de eksplicitte stilarter over det tematiserede træ; når kun `styles` er angivet, erstatter det standarderne helt (den oprindelige adfærd, så eksisterende integrationer og skins påvirkes ikke). `setupDarkModeSkin` er forældet til fordel for `theme`-prop'en.

### Konfigurationsmuligheder

Dette bibliotek sigter mod at understøtte alle konfigurationsmuligheder defineret i [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), ligesom web-implementeringen.

Derudover tilføjer React Native et par SDK-specifikke muligheder via `FastCommentsRNConfig`:

- `hideTopBar` - skjul bjælken med den loggede bruger / notifikationsklokke, der vises over editoren.
- `usePressToEdit` - tryk og hold på en kommentar for at åbne dens menu.
- `disableDownVoting` - skjul nedstem-knapper.
- `renderCommentInline` - render kommentatorinfo i den samme HTML-blok som kommentarindholdet.
- `renderLikesToRight` - flyt stemme/liker-området til højre for kommentaren i stedet for under den.
- `renderDateBelowComment` - render datoen under kommentaren.
- `showLiveStatus` - vis den chat-agtige "Live"-bjælke med brugerantal over kommentarerne.
- `useInlineSubmitButton` - render send-knappen som et ikon inde i editoren.
- `countAboveToggle` - med `useShowCommentsToggle`, hvor mange kommentarer der renderes over "Vis kommentarer"-kontakten.
- `preserveFeedScrollPosition` - `FastCommentsFeed` husker sin scroll-position på tværs af unmount/remount (standard true).

### FastComments-koncepter

De vigtigste koncepter at kende for at komme i gang er `tenantId` og `urlId`. `tenantId` er din FastComments.com-konto-identifikator. `urlId` er det, som kommentartråde
knyttes til. Dette kan være en side-URL, eller et produkt-id, en artikel-id osv.

### Lokalisering

Al brugerrettet tekst i disse widgets (knapetiketter, pladsholdere, tomme tilstande, relative
datoer som "5 minutter siden", fejlbeskeder osv.) er **serverdrevet**. Komponenterne hard-coder
ikke engelske strenge; de renderer de oversættelser, som FastComments serverer for den anmodede
lokalitet.

For at anmode om en lokalitet, sæt `locale` i din config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es osv.
};
```

Når ingen `locale` er sat, serverer FastComments tenantens standardsprog.

**Redigering af teksten:** oversættelser administreres i dit FastComments-dashboard, ikke i denne SDK.
For at ændre formuleringer, tilsidesætte standardteksten eller tilføje et sprog, rediger oversættelserne for din
konto i dashboardet - ændringen bliver automatisk hentet af widgets uden en app-udgivelse. SDK'et leveres uden engelske fallbacks, så enhver nøgle, du tømmer i dashboardet,
renderer tom; hold nøglerne udfyldt for hver lokalitet, du understøtter.

### Brugernotifikationer

FastComments understøtter notifikationer for [mange scenarier](https://docs.fastcomments.com/guide-notifications.html). Notifikationer kan konfigureres,
kan fravælges globalt eller på notifikations-/kommentarniveau, og understøtter side-niveau-abonnementer, så brugere kan abonnere på tråde for en
specifik side eller artikel.

For eksempel er det muligt at bruge Secure SSO til at autentificere brugeren og derefter periodisk polle for ulæste notifikationer og sende dem til brugeren.

Se [eksemplet AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for, hvordan man henter og oversætter ulæste brugernotifikationer.

### Gif Browser

Som standard er intet billede- eller gif-valg aktiveret. Se [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for, hvordan man
understøtter billede- og gif-uploads. Der er en Gif Browser, der anonymiserer søgninger og billeder, som leveres i dette bibliotek - du skal blot bruge den.

### Ydeevne

Venligst opret en ticket med et eksempel til reproduktion, inklusive den anvendte enhed, hvis du identificerer ydeevneproblemer. Ydeevne er en førsteklasses borger
i alle FastComments-biblioteker.