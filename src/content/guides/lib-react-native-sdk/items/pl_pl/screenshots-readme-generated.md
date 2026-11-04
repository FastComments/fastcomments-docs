Komentowanie w wątkach na żywo z awatarami, zagnieżdżonymi odpowiedziami, głosowaniem i wbudowanym edytorem tekstu sformatowanego, a także motyw ciemny i preset czatu na żywo (pokazane tutaj renderowane przez `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Komentowanie na żywo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Komentowanie na żywo, motyw jasny"/></td>
    <td align="center"><b>Motyw ciemny</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Komentowanie na żywo, motyw ciemny"/></td>
    <td align="center"><b>Czat na żywo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset czatu na żywo"/></td>
  </tr>
</table>

### Edytor tekstu sformatowanego

Ta biblioteka używa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) do edycji tekstu sformatowanego, co zapewnia potężne środowisko edycji WYSIWYG. Ten sam edytor obsługuje iOS, Androida i web (przez `react-native-web`), więc kompozytor zachowuje się spójnie na każdej platformie przy jednej implementacji.

`react-native-enriched` wymaga React Native New Architecture (Fabric) na platformach natywnych (domyślne od RN 0.76, opcjonalne w RN 0.72-0.75) oraz bundlera rozwiązującego warunki `exports` pakietów. Ten SDK jest rozwijany i testowany z RN 0.81 / React 19. Ten sam edytor działa również na webie przez `react-native-web`; webowa wersja edytora enriched jest nadal oznaczona upstream jako eksperymentalna.

### Widżety

SDK zawiera trzy widżety, odzwierciedlające FastComments Android SDK:

- `FastCommentsLiveCommenting` - komentowanie w wątkach z głosowaniem, odpowiedziami, paginacją, wzmiankami, powiadomieniami i aktualizacjami na żywo.
- `FastCommentsLiveChat` - preset czatu oparty na tym samym silniku: wiadomości chronologiczne z nowymi na dole, kompozytor pod listą, pasek nagłówka na żywo (kropka połączenia + liczba użytkowników), nieskończona historia ładowana przez przewijanie w górę, automatyczne przewijanie do nowych wiadomości, bez głosowania i wątków odpowiedzi. Każdy preset można nadpisać przez `config`.
- `FastCommentsFeed` - feed społecznościowy z kompozytorem postów, multimediami, reakcjami, obserwowaniem i paskami nowych postów na żywo.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Motywy

Domyślny wygląd jest generowany z zestawu semantycznych tokenów projektowych (`FastCommentsTheme`): kolory, odstępy, promienie, rozmiary czcionek, grubości czcionek i rozmiary awatarów. Przekaż częściowe nadpisania tokenów (typowane `FastCommentsThemeOverrides`) przez prop `theme` na dowolnym widżecie, a całe drzewo stylów zostanie spójnie przestylizowane:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tryb ciemny to tylko jeden zestaw tokenów:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Prop `styles` nadal akceptuje surowe drzewo `IFastCommentsStyles` dla precyzyjnej kontroli. Gdy podane są jednocześnie `theme` i `styles`, jawne style mają pierwszeństwo przed drzewem motywu; gdy podane jest tylko `styles`, całkowicie zastępuje ono domyślne (oryginalne zachowanie, więc istniejące integracje i skiny pozostają nietknięte). `setupDarkModeSkin` jest przestarzałe na rzecz propu `theme`.

### Opcje konfiguracji

Ta biblioteka ma na celu obsługę wszystkich opcji konfiguracji zdefiniowanych w [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), podobnie jak implementacja webowa.

Oprócz tego React Native dodaje kilka opcji specyficznych dla SDK przez `FastCommentsRNConfig`:

- `hideTopBar` - ukrywa pasek zalogowanego użytkownika / dzwonka powiadomień widoczny nad kompozytorem.
- `usePressToEdit` - przytrzymanie komentarza otwiera jego menu.
- `disableDownVoting` - ukrywa przyciski głosowania w dół.
- `renderCommentInline` - renderuje informacje o komentującym w tym samym bloku HTML co treść komentarza.
- `renderLikesToRight` - przenosi obszar głosowania/polubień na prawo od komentarza zamiast pod nim.
- `renderDateBelowComment` - renderuje datę pod komentarzem.
- `showLiveStatus` - pokazuje pasek nagłówka w stylu czatu "Live" + liczba użytkowników nad komentarzami.
- `useInlineSubmitButton` - renderuje przycisk wysyłania jako ikonę wewnątrz kompozytora.
- `countAboveToggle` - z `useShowCommentsToggle`, ile komentarzy renderuje się nad przełącznikiem "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` zapamiętuje pozycję przewijania między odmontowaniem a ponownym zamontowaniem (domyślnie true).

### Koncepcje FastComments

Główne koncepcje, o których trzeba wiedzieć na start, to `tenantId` i `urlId`. `tenantId` to identyfikator Twojego konta FastComments.com. `urlId` to miejsce, z którym będą powiązane wątki komentarzy. Może to być adres URL strony, identyfikator produktu, identyfikator artykułu itp.

### Lokalizacja

Wszystkie teksty widoczne dla użytkownika w tych widżetach (etykiety przycisków, placeholdery, stany puste, daty względne typu "5 minut temu", komunikaty o błędach itp.) są **sterowane przez serwer**. Komponenty nie mają
zaszytych na stałe angielskich ciągów; renderują tłumaczenia, które FastComments serwuje dla żądanej
lokalizacji.

Aby zażądać lokalizacji, ustaw `locale` w swojej konfiguracji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Gdy `locale` nie jest ustawione, FastComments serwuje domyślny język tenanta.

**Edycja tekstów:** tłumaczenia są zarządzane w panelu FastComments, a nie w tym SDK.
Aby zmienić sformułowania, nadpisać domyślne teksty lub dodać język, edytuj tłumaczenia dla swojego
konta w panelu - zmiana jest automatycznie pobierana przez widżety, bez konieczności
wydania aplikacji. SDK nie zawiera angielskich fallbacków, więc każdy klucz, który wyczyścisz w panelu,
renderuje się jako pusty; utrzymuj klucze wypełnione dla każdej obsługiwanej lokalizacji.

### Powiadomienia użytkowników

FastComments obsługuje powiadomienia dla [wielu scenariuszy](https://docs.fastcomments.com/guide-notifications.html). Powiadomienia są konfigurowalne,
można z nich zrezygnować globalnie lub na poziomie powiadomienia/komentarza, a także obsługują subskrypcje na poziomie strony, dzięki czemu użytkownicy mogą subskrybować wątki
konkretnej strony lub artykułu.

Na przykład można użyć Secure SSO do uwierzytelnienia użytkownika, a następnie okresowo odpytywać o nieprzeczytane powiadomienia i wysyłać je do użytkownika.

Zobacz [przykład AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), aby dowiedzieć się, jak pobierać i tłumaczyć nieprzeczytane powiadomienia użytkownika.

### Przeglądarka GIF-ów

Domyślnie wybór obrazów lub GIF-ów nie jest włączony. Zobacz [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), aby dowiedzieć się, jak
obsłużyć przesyłanie obrazów i GIF-ów. W tej bibliotece dostępna jest przeglądarka GIF-ów, która anonimizuje wyszukiwania i obrazy - wystarczy jej użyć.

### Wydajność

Jeśli zauważysz jakiekolwiek problemy z wydajnością, otwórz zgłoszenie (ticket) z przykładem do odtworzenia, w tym z informacją o używanym urządzeniu. Wydajność jest obywatelem pierwszej kategorii
we wszystkich bibliotekach FastComments.