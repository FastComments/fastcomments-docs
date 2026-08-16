Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Komentowanie na żywo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Komentowanie na żywo, jasny motyw"/></td>
    <td align="center"><b>Ciemny motyw</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Komentowanie na żywo, ciemny motyw"/></td>
    <td align="center"><b>Czat na żywo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset czatu na żywo"/></td>
  </tr>
</table>

### Edytor Tekstu Sformatowanego

Ta biblioteka używa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) do edycji tekstu sformatowanego, co zapewnia potężne doświadczenie edycji WYSIWYG. Ten sam edytor napędza iOS, Android oraz web (przez `react-native-web`), więc kompozytor zachowuje się spójnie na każdej platformie przy jednej implementacji.

`react-native-enriched` wymaga nowej architektury React Native (Fabric) na natywnych platformach (domyślnie od RN 0.76, opcjonalnie w RN 0.72‑0.75) oraz bundlera, który rozwiązuje warunki `exports` pakietu. Ten SDK jest rozwijany i testowany pod RN 0.81 / React 19. Ten sam edytor działa również w sieci przez `react-native-web`; wersja web edytora enriched jest nadal oznaczona jako eksperymentalna w upstream.

### Widżety

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` – wątkowane komentarze z głosowaniami, odpowiedziami, paginacją, wzmiankami, powiadomieniami i aktualizacjami na żywo.
- `FastCommentsLiveChat` – preset czatu oparty na tym samym silniku: wiadomości w kolejności chronologicznej, nowe na dole, kompozytor pod listą, pasek nagłówka na żywo (kropka połączenia + liczba użytkowników), nieskończona historia ładowana przewijaniem w górę, automatyczne przewijanie do nowych wiadomości, brak głosowań i wątkowania odpowiedzi. Każdy preset może być nadpisany za pomocą `config`.
- `FastCommentsFeed` – kanał społecznościowy z kompozytorem postów, mediami, reakcjami, obserwacjami i banerami nowo dodanych postów na żywo.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Motywy

Domyślny wygląd jest generowany z zestawu semantycznych tokenów projektowych (`FastCommentsTheme`): kolory, odstępy, promienie, rozmiary czcionek, wagi czcionek i rozmiary awatarów. Przekaż częściowe nadpisania tokenów (typowane jako `FastCommentsThemeOverrides`) przez właściwość `theme` w dowolnym widżecie, a cały drzewo stylów zostanie spójnie przestylizowane:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tryb ciemny jest oddalony o jeden zestaw tokenów:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Właściwość `styles` nadal akceptuje surowe drzewo `IFastCommentsStyles` dla precyzyjnej kontroli. Gdy podane są zarówno `theme`, jak i `styles`, explicite style mają pierwszeństwo przed drzewem tematycznym; gdy podane są tylko `styles`, zastępują one całkowicie domyślne (pierwotne zachowanie, więc istniejące integracje i skiny pozostają niezmienione). `setupDarkModeSkin` jest przestarzałe na rzecz właściwości `theme`.

### Opcje Konfiguracji

Ta biblioteka ma na celu obsługę wszystkich opcji konfiguracyjnych zdefiniowanych w [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tak jak implementacja webowa.

On top of those, React Native adds a few SDK-specific options via `FastCommentsRNConfig`:

- `hideTopBar` – ukrywa pasek zalogowanego użytkownika / dzwonek powiadomień wyświetlany nad kompozytorem.
- `usePressToEdit` – przytrzymaj komentarz, aby otworzyć jego menu.
- `disableDownVoting` – ukrywa przyciski głosowania w dół.
- `renderCommentInline` – renderuje informacje o komentującym wewnątrz tego samego bloku HTML co treść komentarza.
- `renderLikesToRight` – przenosi obszar głosowania/polubień na prawo od komentarza zamiast pod nim.
- `renderDateBelowComment` – wyświetla datę pod komentarzem.
- `showLiveStatus` – wyświetla pasek nagłówka w stylu czatu „Live” + liczba użytkowników nad komentarzami.
- `useInlineSubmitButton` – renderuje przycisk wysyłania jako ikonę wewnątrz kompozytora.
- `countAboveToggle` – wraz z `useShowCommentsToggle`, określa, ile komentarzy renderować powyżej przełącznika „Pokaż komentarze”.
- `preserveFeedScrollPosition` – `FastCommentsFeed` zapamiętuje pozycję przewijania pomiędzy odmontowaniem a ponownym zamontowaniem (domyślnie true).

### Koncepcje FastComments

Główne koncepcje, o których należy wiedzieć, aby rozpocząć, to `tenantId` i `urlId`. `tenantId` jest identyfikatorem Twojego konta FastComments.com. `urlId` określa, do czego będą powiązane wątki komentarzy. Może to być adres URL strony, identyfikator produktu, identyfikator artykułu itp.

### Lokalizacja

Wszystkie teksty widoczne dla użytkownika w tych widżetach (etykiety przycisków, placeholdery, stany pustych danych, względne daty jak „5 minut temu”, komunikaty o błędach itp.) są **sterowane przez serwer**. Komponenty nie mają zakodowanych na stałe angielskich ciągów; renderują tłumaczenia, które FastComments udostępnia dla żądanego języka.

Aby żądać konkretnego języka, ustaw `locale` w swojej konfiguracji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Gdy nie ustawiono `locale`, FastComments używa domyślnego języka najemcy.

**Edycja tekstu:** tłumaczenia są zarządzane w panelu FastComments, a nie w tym SDK. Aby zmienić sformułowanie, nadpisz domyślną treść lub dodaj język, edytuj tłumaczenia dla swojego konta w panelu – zmiana jest automatycznie wykrywana przez widżety bez konieczności wydania aplikacji. SDK nie dostarcza angielskich wersji zapasowych, więc każdy klucz, który pozostawisz pusty w panelu, będzie wyświetlał pustą wartość; utrzymuj klucze wypełnione dla każdego obsługiwanego języka.

### Powiadomienia Użytkownika

FastComments obsługuje powiadomienia dla [wielu scenariuszy](https://docs.fastcomments.com/guide-notifications.html). Powiadomienia są konfigurowalne, można je wyłączyć globalnie lub na poziomie powiadomienia/komentarza, oraz obsługują subskrypcje na poziomie strony, dzięki czemu użytkownicy mogą subskrybować wątki konkretnej strony lub artykułu.

Na przykład, można użyć Secure SSO do uwierzytelnienia użytkownika, a następnie okresowo odpytywać o nieprzeczytane powiadomienia i przesyłać je do użytkownika.

Zobacz [przykład AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) aby dowiedzieć się, jak pobrać i przetłumaczyć nieprzeczytane powiadomienia użytkownika.

### Przeglądarka GIF

Domyślnie nie jest włączony żaden wybór obrazów ani gifów. Zobacz [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), aby dowiedzieć się, jak obsługiwać przesyłanie obrazów i gifów. Istnieje Przeglądarka GIF, która anonimizuje wyszukiwania i obrazy udostępnione w tej bibliotece; wystarczy ją użyć.

### Wydajność

Prosimy o otwarcie zgłoszenia z przykładem do odtworzenia, w tym używanym urządzeniem, jeśli zidentyfikujesz jakiekolwiek problemy z wydajnością. Wydajność jest priorytetem we wszystkich bibliotekach FastComments.