Live gestructureerde reacties met avatars, geneste antwoorden, stemmen, en de ingebouwde rich‑text composer, plus een donker thema en een live‑chat preset (hier weergegeven via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Reacties</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live reacties, licht thema"/></td>
    <td align="center"><b>Donker thema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live reacties, donker thema"/></td>
    <td align="center"><b>Live chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live-chat preset"/></td>
  </tr>
</table>

### Rich Text-editor

Deze bibliotheek maakt gebruik van [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) voor rich‑textbewerking, die een krachtige WYSIWYG‑bewerkingservaring biedt. Dezelfde editor ondersteunt iOS, Android en het web (via `react-native-web`), zodat de composer consistent werkt op elk platform met een enkele implementatie.

`react-native-enriched` vereist de React Native New Architecture (Fabric) op native (standaard sinds RN 0.76, opt‑in op RN 0.72‑0.75), en een bundler die package `exports`‑condities oplost. Deze SDK is ontwikkeld en getest tegen RN 0.81 / React 19. Dezelfde editor draait ook op het web via `react-native-web`; de web‑build van de enriched editor wordt nog steeds als experimenteel gemarkeerd bij de upstream.

### Widgets

De SDK levert drie widgets, die overeenkomen met de FastComments Android SDK:

- `FastCommentsLiveCommenting` - gestructureerde reacties met stemmen, antwoorden, paginering, vermeldingen, meldingen en live‑updates.
- `FastCommentsLiveChat` - een chat‑preset op dezelfde engine: chronologische berichten met nieuwe onderaan, de composer onder de lijst, een live‑headerbalk (verbindingstip + aantal gebruikers), oneindige geschiedenis die wordt geladen door omhoog te scrollen, automatisch scrollen naar nieuwe berichten, geen stemmen of antwoordstructuur. Elk preset kan worden overschreven via `config`.
- `FastCommentsFeed` - een sociale feed met post‑composer, media, reacties, volgers, en live banners voor nieuwe berichten.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Thema's

De standaarduiterlijk wordt gegenereerd uit een set semantische ontwerp‑tokens (`FastCommentsTheme`): kleuren, spacing, radius, lettergroottes, lettergewicht en avatar‑groottes. Geef gedeeltelijke token‑overschrijvingen (getypeerd als `FastCommentsThemeOverrides`) door via de `theme`‑prop op elk widget en de volledige stijlboom wordt consistent opnieuw gestyled:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Dark‑mode is één token‑set verwijderd:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

De `styles`‑prop accepteert nog steeds een ruwe `IFastCommentsStyles`‑boom voor chirurgische controle. Wanneer zowel `theme` als `styles` worden opgegeven, hebben de expliciete stijlen voorrang op de thematische boom; wanneer alleen `styles` wordt opgegeven, vervangt het de standaardinstellingen volledig (het oorspronkelijke gedrag, zodat bestaande integraties en skins ongewijzigd blijven). `setupDarkModeSkin` is verouderd ten gunste van de `theme`‑prop.

### Configuratie‑opties

Deze bibliotheek streeft ernaar alle configuratie‑opties te ondersteunen die gedefinieerd zijn in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), net als de web‑implementatie.

Naast die opties voegt React Native een paar SDK‑specifieke opties toe via `FastCommentsRNConfig`:

- `hideTopBar` - verbergt de bovenbalk met ingelogde gebruiker / meldingsbel die boven de composer wordt getoond.
- `usePressToEdit` - druk‑en‑houd een reactie om het menu te openen.
- `disableDownVoting` - verbergt down‑vote knoppen.
- `renderCommentInline` - render de informatie van de reageerder binnen hetzelfde HTML‑blok als de commentaarinhoud.
- `renderLikesToRight` - verplaatst het stem/like‑gebied naar rechts van de reactie in plaats van eronder.
- `renderDateBelowComment` - toont de datum onder de reactie.
- `showLiveStatus` - toont de chat‑stijl "Live" + gebruikers‑aantal header‑balk boven reacties.
- `useInlineSubmitButton` - rendert de verzendknop als een icoon binnen de composer.
- `countAboveToggle` - in combinatie met `useShowCommentsToggle`, hoeveel reacties worden weergegeven boven de "Show Comments"‑schakelaar.
- `preserveFeedScrollPosition` - `FastCommentsFeed` onthoudt zijn scroll‑offset bij unmount/remount (standaard true).

### FastComments-concepten

De belangrijkste concepten om kennis van te hebben bij het starten zijn `tenantId` en `urlId`. `tenantId` is de identifier van uw FastComments.com‑account. `urlId` is waar de reactie‑threads aan gekoppeld worden. Dit kan een paginapad, een product‑id, een artikel‑id, enzovoort zijn.

### Lokalisatie

Alle tekst die aan de gebruiker wordt getoond in deze widgets (knop‑labels, placeholders, lege statussen, relatieve datums zoals "5 minuten geleden", foutmeldingen, enz.) wordt **server‑gedreven**. De componenten coderen geen Engelse strings hard‑coded; ze renderen de vertalingen die FastComments levert voor de gevraagde locale.

Om een locale aan te vragen, stel `locale` in uw config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Wanneer geen `locale` is ingesteld, levert FastComments de standaardtaal van de tenant.

**Tekst bewerken:** vertalingen worden beheerd in uw FastComments‑dashboard, niet in deze SDK. Om de bewoording te wijzigen, overschrijf de standaardtekst, of voeg een taal toe, bewerk de vertalingen voor uw account in het dashboard – de wijziging wordt automatisch opgepikt door de widgets zonder dat een app‑release nodig is. De SDK levert geen Engelse fallback‑teksten, dus elke sleutel die u leegmaakt in het dashboard wordt leeg weergegeven; houd de sleutels gevuld voor elke locale die u ondersteunt.

### Gebruikersmeldingen

FastComments ondersteunt meldingen voor [veel scenario's](https://docs.fastcomments.com/guide-notifications.html). Meldingen zijn configureerbaar, kunnen globaal of op meldings-/commentaar‑niveau worden uitgeschakeld, en ondersteunen abonnementen op paginaniveau zodat gebruikers zich kunnen abonneren op threads van een specifieke pagina of artikel.

Bijvoorbeeld, het is mogelijk Secure SSO te gebruiken om de gebruiker te authenticeren en vervolgens periodiek te pollen voor ongelezen meldingen en deze naar de gebruiker te pushen.

Zie [het voorbeeld AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) voor hoe ongelezen gebruikersmeldingen te verkrijgen en vertalen.

### Gif-browser

Standaard is geen afbeelding of gif‑selectie ingeschakeld. Zie [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) voor hoe image‑ en gif‑uploads te ondersteunen. Er is een Gif‑browser die zoekopdrachten en afbeeldingen die in deze bibliotheek worden geleverd anonimiseert; u hoeft deze alleen te gebruiken.

### Prestaties

Open alstublieft een ticket met een voorbeeld om te reproduceren, inclusief het gebruikte apparaat, als u prestatieproblemen identificeert. Prestaties zijn een belangrijk aspect van alle FastComments‑bibliotheken.