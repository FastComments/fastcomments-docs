### Obsługa trybu ciemnego

Image Chat zawiera wbudowaną obsługę trybu ciemnego. Gdy ustawisz `hasDarkBackground: true` w swojej konfiguracji, okna czatu i elementy interfejsu automatycznie dostosują się do ciemnych tł.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Stylizacja trybu ciemnego dotyczy okien czatu, kwadratowych znaczników i wszystkich elementów interaktywnych. Jeśli Twoja witryna ma przełącznik trybu ciemnego, możesz ponownie zainicjować widżet po zmianie trybu lub użyć podejścia z klasą body opisanego poniżej.

### Dynamiczny tryb ciemny

Jeśli tryb ciemny Twojej witryny jest kontrolowany przez dodanie klasy `.dark` do elementu body, interfejs Image Chat automatycznie to uwzględni bez konieczności ponownej inicjalizacji. Style widżetu zostały zaprojektowane tak, aby reagować na obecność tej klasy.

```css
/* Twoje CSS trybu ciemnego */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Niestandardowa stylizacja za pomocą CSS

Możesz dostosować wygląd znaczników, okien czatu i innych elementów za pomocą CSS. Widżet dodaje określone klasy, które możesz wykorzystać w swoim arkuszu stylów.

Kwadraty i okna czatu korzystają z systemu stylizacji dymków komentarzy FastComments, więc wszelkie dostosowania zastosowane do standardowego widżetu komentarzy będą miały również wpływ na Image Chat.

### Rozmiar kwadratów czatu

Opcja `chatSquarePercentage` kontroluje rozmiar klikalnych znaczników. Wartość domyślna to 5% szerokości obrazu:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Większe, bardziej widoczne kwadraty
});
```

Mniejsze wartości tworzą bardziej subtelne znaczniki, które wtapiają się w obraz. Większe wartości sprawiają, że znaczniki są bardziej widoczne i łatwiejsze do kliknięcia, szczególnie na urządzeniach mobilnych lub ze względów dostępności.

### Zachowanie na urządzeniach mobilnych

Na ekranach o szerokości poniżej 768px Image Chat automatycznie przełącza się na układ zoptymalizowany pod kątem urządzeń mobilnych. Okna czatu pojawiają się na pełnym ekranie zamiast unosić się obok znaczników, co zapewnia lepszą użyteczność na małych ekranach.

Znaczniki pozostają widoczne w swoich responsywnych pozycjach na obrazie. Użytkownicy mogą dotknąć dowolnego znacznika, aby otworzyć pełnoekranowy interfejs czatu. To zachowanie jest wbudowane i nie wymaga żadnej konfiguracji.

### Wygląd okna czatu

Okna czatu mają 300px szerokości na komputerach stacjonarnych ze strzałką 16px wskazującą znacznik. Okna pozycjonują się automatycznie w zależności od dostępnej przestrzeni w oknie widoku, używając klas pozycjonowania takich jak `to-right`, `to-left`, `to-top` i `to-bottom`.

Możesz dodać niestandardowy CSS, aby dostosować kolory, czcionki, odstępy lub inne właściwości wizualne tych okien. Okna czatu używają tej samej struktury komponentów co standardowy widżet FastComments, więc dziedziczą wszelkie globalne dostosowania, które zastosowałeś.

### Leniwa inicjalizacja

Okna czatu inicjalizują się po najechaniu kursorem dla użytkowników komputerów stacjonarnych lub natychmiast po utworzeniu. Zmniejsza to początkowe obciążenie ładowania, renderując interfejs czatu tylko wtedy, gdy użytkownicy faktycznie wchodzą w interakcję ze znacznikiem.

Leniwa inicjalizacja odbywa się w sposób niewidoczny. Użytkownicy nie zauważają żadnego opóźnienia, ale przeglądarka nie musi renderować dziesiątek ukrytych okien czatu, jeśli na obrazie znajduje się wiele znaczników.

### Lokalizacja

Image Chat obsługuje te same opcje lokalizacji co standardowy widżet FastComments. Ustaw opcję `locale`, aby wyświetlać tekst interfejsu w różnych językach:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francuski
});
```

FastComments obsługuje dziesiątki języków. Ustawienie locale wpływa na cały tekst interfejsu, w tym podpowiedzi, przyciski i tekst zastępczy.

### Odziedziczone opcje dostosowywania

Ponieważ Image Chat rozszerza standardowy widżet komentarzy, dziedziczy wszystkie opcje dostosowywania z bazowego widżetu. Obejmuje to niestandardowe klasy CSS, niestandardowe tłumaczenia, dostosowywanie awatarów, formatowanie dat i wiele więcej.

Zobacz główną dokumentację dostosowywania FastComments, aby zapoznać się z pełną listą dostępnych opcji dostosowywania.

### Praca z niestandardowymi czcionkami

Jeśli Twoja witryna używa niestandardowych czcionek, interfejs Image Chat odziedziczy te czcionki z CSS Twojej strony. Okna czatu są renderowane wewnątrz DOM Twojej strony i respektują istniejące ustawienia typografii.

Aby uzyskać najlepsze rezultaty, upewnij się, że niestandardowe czcionki są załadowane przed zainicjowaniem Image Chat, albo zaakceptuj, że może wystąpić krótki błysk tekstu bez stylizacji podczas ładowania czcionek.

### Projekt wizualny znaczników

Kwadratowe znaczniki mają subtelny projekt wizualny, który sprawia, że są zauważalne, nie przytłaczając obrazu. Możesz dostosować ich wygląd za pomocą CSS, jeśli chcesz uzyskać inny efekt wizualny.

Znaczniki zawierają stany hover, które zapewniają informację zwrotną, gdy użytkownicy przesuwają nad nimi kursor. Na urządzeniach dotykowych interakcja dotknięcia zapewnia natychmiastową informację zwrotną poprzez otwarcie okna czatu.