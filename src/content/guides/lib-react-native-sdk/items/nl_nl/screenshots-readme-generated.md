Live threaded commentaar met avatars, geneste antwoorden, stemmen en de ingebouwde rich‑text composer, plus een donker thema en een live‑chat preset (hier weergegeven via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live commentaar</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Live commentaar, licht thema"/></td>
    <td align="center"><b>Donker thema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Live commentaar, donker thema"/></td>
    <td align="center"><b>Live chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich‑tekst editor

Deze bibliotheek gebruikt [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) voor rich‑text bewerking, die een krachtige WYSIWYG‑bewerkingservaring biedt. Dezelfde editor werkt op iOS, Android en het web (via `react-native-web`), zodat de composer consistent werkt op elk platform met één implementatie.

`react-native-enriched` vereist de React Native New Architecture (Fabric) op native (standaard sinds RN 0.76, opt‑in op RN 0.72‑0.75), en een bundler die package `exports`‑condities oplost. Deze SDK is ontwikkeld en getest tegen RN 0.81 / React 19. Dezelfde editor draait ook op het web via `react-native-web`; de web‑build van de enriched editor wordt nog steeds als experimenteel gemarkeerd upstream.

### Widgets

De SDK levert drie widgets, die de FastComments Android SDK weerspiegelen:

- `FastCommentsLiveCommenting` - threaded commentaar met stemmen, antwoorden, paginering, vermeldingen, meldingen en live‑updates.
- `FastCommentsLiveChat` - een chat‑preset op dezelfde engine: chronologische berichten met nieuwe onderaan, de composer onder de lijst, een live header‑strip (verbindingstip + gebruikersaantal), oneindige geschiedenis geladen door omhoog te scrollen, automatisch scrollen naar nieuwe berichten, geen stemmen of antwoord‑threading. Elke preset kan worden overschreven via `config`.
- `FastCommentsFeed` - een sociale feed met post‑composer, media, reacties, volgers, en live nieuw‑post banners.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Thema's

De standaarduiterlijk wordt gegenereerd uit een set semantische design‑tokens (`FastCommentsTheme`): kleuren, spacing, radius, lettergroottes, lettergewichten en avatar‑groottes. Geef gedeeltelijke token‑overrides (getypeerd `FastCommentsThemeOverrides`) door via de `theme`‑prop op elk widget en de volledige stijlbomen worden consistent opnieuw gestyled:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Donkere modus is één token‑set verwijderd:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

De `styles`‑prop accepteert nog steeds een ruwe `IFastCommentsStyles`‑boom voor chirurgische controle. Wanneer zowel `theme` als `styles` worden opgegeven, hebben de expliciete stijlen voorrang op de thematische boom; wanneer alleen `styles` wordt opgegeven, vervangt deze volledig de standaardinstellingen (het oorspronkelijke gedrag, zodat bestaande integraties en skins onaangedaan blijven). `setupDarkModeSkin` is verouderd ten gunste van de `theme`‑prop.

### Configuratie‑opties

Deze bibliotheek streeft ernaar alle configuratie‑opties te ondersteunen die zijn gedefinieerd in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), net als de web‑implementatie.

Bovenop die, voegt React Native een paar SDK‑specifieke opties toe via `FastCommentsRNConfig`:

- `hideTopBar` - verberg de balk met ingelogde gebruiker / meldingsbel die boven de composer wordt getoond.
- `usePressToEdit` - druk‑en‑houd een reactie om het menu te openen.
- `disableDownVoting` - verberg neer‑stemknoppen.
- `renderCommentInline` - render de informatie van de reageerder binnen hetzelfde HTML‑blok als de commentaarinhoud.
- `renderLikesToRight` - verplaats het stem/like‑gebied naar rechts van de reactie in plaats van eronder.
- `renderDateBelowComment` - render de datum onder de reactie.
- `showLiveStatus` - toon de chat‑stijl "Live" + gebruikers‑aantal header‑strip boven reacties.
- `useInlineSubmitButton` - render de verzendknop als een icoon binnen de composer.
- `countAboveToggle` - met `useShowCommentsToggle`, hoeveel reacties worden weergegeven boven de "Show Comments"‑schakelaar.
- `preserveFeedScrollPosition` - `FastCommentsFeed` onthoudt zijn scroll‑offset over unmount/remount heen (standaard true).

### FastComments-concepten

De belangrijkste concepten om mee te beginnen zijn `tenantId` en `urlId`. `tenantId` is de identificatie van uw FastComments.com‑account. `urlId` is waar de commentaarthreads aan gekoppeld worden. Dit kan een pagin URL zijn, of een product‑id, een artikel‑id, enz.

### Lokalisatie

Alle gebruikersgerichte tekst in deze widgets (knop‑labels, placeholders, lege staten, relatieve datums zoals "5 minuten geleden", foutmeldingen, enz.) is **server‑gedreven**. De componenten coderen geen Engelse strings; ze renderen de vertalingen die FastComments levert voor de gevraagde locale.

Om een locale aan te vragen, stel `locale` in uw config in:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Wanneer geen `locale` is ingesteld, levert FastComments de standaardtaal van de tenant.

**Tekst bewerken:** vertalingen worden beheerd in uw FastComments‑dashboard, niet in deze SDK. Om de bewoording te wijzigen, overschrijf de standaardtekst, of voeg een taal toe, bewerk de vertalingen voor uw account in het dashboard – de wijziging wordt automatisch opgepikt door de widgets zonder dat een app‑release nodig is. De SDK levert geen Engelse fallback‑teksten, dus elke sleutel die u leeg laat in het dashboard wordt leeg gerenderd; houd de sleutels gevuld voor elke locale die u ondersteunt.

### Gebruikersmeldingen

FastComments ondersteunt meldingen voor [veel scenario's](https://docs.fastcomments.com/guide-notifications.html). Meldingen zijn configureerbaar, kunnen globaal of op meldings-/reactieniveau worden uitgeschakeld, en ondersteunen abonnementen op paginaniveau zodat gebruikers zich kunnen abonneren op threads van een specifieke pagina of artikel.

Bijvoorbeeld, het is mogelijk Secure SSO te gebruiken om de gebruiker te authenticeren en vervolgens periodiek te pollen voor ongelezen meldingen en deze naar de gebruiker te pushen.

Zie [het voorbeeld AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) voor hoe ongelezen gebruikersmeldingen te verkrijgen en te vertalen.

### Gif-browser

Standaard is geen afbeelding‑ of gif‑selectie ingeschakeld. Zie [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) voor hoe u afbeelding‑ en gif‑uploads kunt ondersteunen. Er is een Gif‑browser die zoekopdrachten en afbeeldingen in deze bibliotheek anonimiseert; u hoeft deze alleen maar te gebruiken.

### Prestaties

Open alstublieft een ticket met een voorbeeld om te reproduceren, inclusief het gebruikte apparaat, als u prestatieproblemen identificeert. Prestaties zijn een first‑class citizen van alle FastComments‑bibliotheken.