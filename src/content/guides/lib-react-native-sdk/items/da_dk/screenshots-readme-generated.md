Live trådede kommentarer med avatars, indlejrede svar, stemmer og den indbyggede rich-text-komponist, plus et mørkt tema og en live-chat-forudindstilling (vist her gengivet via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live-kommentering</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live-kommentering, lyst tema"/></td>
    <td align="center"><b>Mørkt tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live-kommentering, mørkt tema"/></td>
    <td align="center"><b>Live-chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live-chat-forudindstilling"/></td>
  </tr>
</table>

### Rich Text Editor

Dette bibliotek bruger [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) til rich text-redigering, hvilket giver en kraftfuld WYSIWYG-redigeringsoplevelse. Den samme editor driver iOS, Android og web (via `react-native-web`), så komponisten opfører sig konsekvent på tværs af alle platforme med én enkelt implementation.

`react-native-enriched` kræver React Native New Architecture (Fabric) på native (standard siden RN 0.76, opt-in på RN 0.72-0.75), samt en bundler der løser package `exports`-betingelser. Dette SDK er udviklet og testet mod RN 0.81 / React 19. Den samme editor kører også på web via `react-native-web`; enriched-editorens web-build er stadig markeret som eksperimentel upstream.

### Widgets

SDK'et leveres med tre widgets, som spejler FastComments Android SDK:

- `FastCommentsLiveCommenting` - trådet kommentering med stemmer, svar, paginering, mentions, notifikationer og live-opdateringer.
- `FastCommentsLiveChat` - en chat-forudindstilling over samme motor: kronologiske beskeder med nye nederst, komponisten under listen, en live header-strip (forbindelsesprik + brugerantal), uendelig historik indlæst ved at scrolle op, auto-scroll til nye beskeder, ingen stemmer eller svar-tråde. Hver forudindstilling kan overskrives via `config`.
- `FastCommentsFeed` - et socialt feed med indholds-komponist, medier, reaktioner, følgere og live bannere for nye opslag.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Standardudseendet genereres fra et sæt semantiske designtokens (`FastCommentsTheme`): farver, spacing, radius, fontstørrelser, fontvægte og avatarstørrelser. Send delvise token-overrides (typet `FastCommentsThemeOverrides`) gennem `theme`-props på enhver widget, og hele stiltræet restyles konsekvent:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Mørk tilstand er kun et token-sæt væk:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles`-proppen accepterer stadig et råt `IFastCommentsStyles`-træ for kirurgisk kontrol. Når både `theme` og `styles` leveres, vinder de eksplicitte styles over det themed træ; når kun `styles` leveres, erstatter det standarderne fuldstændigt (den oprindelige adfærd, så eksisterende integrationer og skins påvirkes ikke). `setupDarkModeSkin` er forældet til fordel for `theme`-proppen.

### Configuration Options

Dette bibliotek har til formål at understøtte alle konfigurationsmuligheder defineret i [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), ligesom web-implementeringen.

Udover disse tilføjer React Native nogle få SDK-specifikke muligheder via `FastCommentsRNConfig`:

- `hideTopBar` - skjul strippen med den indloggede bruger / notifikationsklokke, som vises over komponisten.
- `usePressToEdit` - tryk-og-hold på en kommentar for at åbne dens menu.
- `disableDownVoting` - skjul nedstemme-knapper.
- `renderCommentInline` - gengiv kommentatorinfo inde i samme HTML-blok som kommentarindholdet.
- `renderLikesToRight` - flyt stemme/like-området til højre for kommentaren i stedet for under den.
- `renderDateBelowComment` - gengiv datoen under kommentaren.
- `showLiveStatus` - vis chat-stil "Live" + brugerantal header-strip over kommentarer.
- `useInlineSubmitButton` - gengiv sendeknappen som et ikon inde i komponisten.
- `countAboveToggle` - med `useShowCommentsToggle`, hvor mange kommentarer der gengives over "Show Comments"-toggle.
- `preserveFeedScrollPosition` - `FastCommentsFeed` husker sin scroll-offset på tværs af unmount/remount (standard true).

### FastComments Concepts

De vigtigste begreber at kende for at komme i gang er `tenantId` og `urlId`. `tenantId` er din FastComments.com kontoidentifikator. `urlId` er det, som kommentartråde vil blive knyttet til. Dette kan være en side-URL, et produkt-id, et artikel-id osv.

### User Notifications

FastComments understøtter notifikationer for [mange scenarier](https://docs.fastcomments.com/guide-notifications.html). Notifikationer er konfigurerbare, kan fravælges globalt eller på notifikation/kommentar-niveau, og understøtter side-niveau abonnementer, så brugere kan abonnere på tråde for en bestemt side eller artikel.

For eksempel er det muligt at bruge Secure SSO til at autentificere brugeren og derefter periodisk poll'e for ulæste notifikationer og skubbe dem til brugeren.

Se [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for hvordan man henter og oversætter ulæste bruger-notifikationer.

### Gif Browser

Som standard er ingen billed- eller gif-selektion aktiveret. Se [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for hvordan
man understøtter billede- og gif-upload. Der er en Gif Browser i dette bibliotek, som anonymiserer søgninger og billeder leveret i dette bibliotek — du skal blot bruge den.

### Performance

Opret venligst en ticket med et eksempel der kan reproduceres, inklusive hvilken enhed der blev brugt, hvis du identificerer nogen performance-problemer. Performance er en topprioritet i alle FastComments-biblioteker.