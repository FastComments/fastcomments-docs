Live reactie-threads met avatars, geneste reacties, stemmen en de ingebouwde rich-text composer, plus een donker thema en een live-chat preset (hier getoond gerenderd via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text Editor

Deze library gebruikt [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) voor rich-text editing, wat een krachtige WYSIWYG-bewerkingservaring biedt. Dezelfde editor werkt op iOS, Android en het web (via `react-native-web`), zodat de composer zich op elk platform consistent gedraagt met één implementatie.

`react-native-enriched` vereist de React Native New Architecture (Fabric) op native (standaard sinds RN 0.76, opt-in op RN 0.72-0.75), en een bundler die package `exports`-condities oplost. Deze SDK is ontwikkeld en getest tegen RN 0.81 / React 19. Dezelfde editor draait ook op het web via `react-native-web`; de web-build van de enriched editor is upstream nog steeds gemarkeerd als experimenteel.

### Widgets

De SDK bevat drie widgets, vergelijkbaar met de FastComments Android SDK:

- `FastCommentsLiveCommenting` - reactie-threads met stemmen, replies, paginering, mentions, notificaties en live updates.
- `FastCommentsLiveChat` - een chat-preset op dezelfde engine: chronologische berichten met nieuwe onderaan, de composer onder de lijst, een live header-balk (verbindingspunt + aantal gebruikers), oneindige geschiedenis door omhoog te scrollen, automatisch scrollen naar nieuwe berichten, geen stemmen of reply-threading. Elke preset kan worden overschreven via `config`.
- `FastCommentsFeed` - een social feed met post-composer, media, reacties, follows en live banners voor nieuwe posts.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

De standaardweergave wordt gegenereerd uit een set semantische design tokens (`FastCommentsTheme`): kleuren, spacing, radius, lettergroottes, lettergewichten en avatar-groottes. Geef partiële token-overschrijvingen door (getypt als `FastCommentsThemeOverrides`) via de `theme`-prop op elke widget en de hele stijlboom wordt consistent herstijld:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Dark mode is één token-set verwijderd:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

De `styles`-prop accepteert nog steeds een ruwe `IFastCommentsStyles`-boom voor chirurgische controle. Wanneer zowel `theme` als `styles` worden meegegeven, winnen de expliciete styles van de gethematiseerde boom; wanneer alleen `styles` wordt meegegeven, vervangt deze de standaarden volledig (het oorspronkelijke gedrag, dus bestaande integraties en skins worden niet beïnvloed). `setupDarkModeSkin` is verouderd ten gunste van de `theme`-prop.

### Configuratieopties

Deze library streeft ernaar alle configuratieopties te ondersteunen die zijn gedefinieerd in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), net als de web-implementatie.

Daarbovenop voegt React Native een paar SDK-specifieke opties toe via `FastCommentsRNConfig`:

- `hideTopBar` - verberg de balk met ingelogde gebruiker / notificatiebel die boven de composer wordt getoond.
- `usePressToEdit` - houd een reactie ingedrukt om het menu te openen.
- `disableDownVoting` - verberg de downvote-knoppen.
- `renderCommentInline` - render de informatie van de reageerder binnen hetzelfde HTML-blok als de inhoud van de reactie.
- `renderLikesToRight` - verplaats het stem/like-gebied naar de rechterkant van de reactie in plaats van eronder.
- `renderDateBelowComment` - render de datum onder de reactie.
- `showLiveStatus` - toon de chat-achtige "Live"-balk met aantal gebruikers boven de reacties.
- `useInlineSubmitButton` - render de verzendknop als een icoon binnen de composer.
- `countAboveToggle` - met `useShowCommentsToggle`, hoeveel reacties er boven de "Show Comments"-toggle worden gerenderd.
- `preserveFeedScrollPosition` - `FastCommentsFeed` onthoudt zijn scrollpositie tussen unmount/remount (standaard true).

### FastComments Concepten

De belangrijkste concepten om mee te beginnen zijn `tenantId` en `urlId`. `tenantId` is de identificatie van je FastComments.com-account. `urlId` bepaalt waaraan reactie-threads worden gekoppeld. Dit kan een pagina-URL zijn, of een product-id, een artikel-id, enz.

### Lokalisatie

Alle gebruikersgerichte tekst in deze widgets (knoplabels, placeholders, lege statussen, relatieve
datums zoals "5 minuten geleden", foutmeldingen, enz.) is **servergestuurd**. De componenten
hardcoden geen Engelse strings; ze renderen de vertalingen die FastComments levert voor de
opgevraagde locale.

Om een locale op te vragen, stel `locale` in je config in:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, enz.
};
```

Wanneer er geen `locale` is ingesteld, levert FastComments de standaardtaal van de tenant.

**Tekst bewerken:** vertalingen worden beheerd in je FastComments-dashboard, niet in deze SDK.
Om formuleringen te wijzigen, de standaardtekst te overschrijven of een taal toe te voegen, bewerk je de vertalingen voor je
account in het dashboard - de wijziging wordt automatisch door de widgets overgenomen zonder dat een
app-release nodig is. De SDK bevat geen Engelse fallbacks, dus elke key die je in het dashboard leegmaakt
rendert leeg; houd de keys gevuld voor elke locale die je ondersteunt.

### Gebruikersnotificaties

FastComments ondersteunt notificaties voor [veel scenario's](https://docs.fastcomments.com/guide-notifications.html). Notificaties zijn configureerbaar,
kunnen globaal of op notificatie/reactie-niveau worden uitgezet, en ondersteunen paginaniveau-abonnementen zodat gebruikers zich kunnen abonneren op threads van een
specifieke pagina of artikel.

Het is bijvoorbeeld mogelijk om Secure SSO te gebruiken om de gebruiker te authenticeren en vervolgens periodiek ongelezen notificaties op te halen en naar de gebruiker te pushen.

Zie [het voorbeeld AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) voor hoe je ongelezen gebruikersnotificaties ophaalt en verwerkt.

### Gif Browser

Standaard is geen afbeeldings- of gif-selectie ingeschakeld. Zie [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) voor hoe je
uploads van afbeeldingen en gifs kunt ondersteunen. Er is een Gif Browser die zoekopdrachten en aangeleverde afbeeldingen anonimiseert in deze library; je hoeft hem alleen te gebruiken.

### Performance

Open een ticket met een voorbeeld om het probleem te reproduceren, inclusief het gebruikte apparaat, als je prestatieproblemen constateert. Performance is een eerste-class burger
in alle FastComments-libraries.