### Obsługa trybu ciemnego

### Dynamiczny tryb ciemny

Jeśli tryb ciemny na Twojej stronie jest kontrolowany przez dodanie klasy `.dark` do elementu body, interfejs Collab Chat automatycznie go uwzględni bez konieczności ponownej inicjalizacji. Style widżetu są zaprojektowane tak, aby reagować na obecność tej klasy.

[inline-code-attrs-start title = 'Przykład CSS dla trybu ciemnego'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Twój CSS dla trybu ciemnego */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Niestandardowe style za pomocą CSS

Możesz dostosować wygląd wyróżnień, okien czatu i innych elementów za pomocą CSS. Widżet dodaje określone klasy, które możesz adresować w swoim arkuszu stylów.

Wyróżnienia tekstu korzystają z systemu stylów dymków komentarzy FastComments, więc wszelkie modyfikacje zastosowane do standardowego widżetu komentarzy będą również wpływać na Collab Chat.

### Dostosowywanie górnego paska

Górny pasek pokazuje liczbę użytkowników online oraz liczbę dyskusji. Możesz dostosować jego położenie, podając niestandardowy element jako `topBarTarget`:

[inline-code-attrs-start title = 'Niestandardowa lokalizacja górnego paska'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Lub całkowicie go wyłączyć ustawiając wartość na `null`:

[inline-code-attrs-start title = 'Wyłącz górny pasek'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Zachowanie na urządzeniach mobilnych

Na ekranach o szerokości poniżej 768px, Collab Chat automatycznie przełącza się na układ zoptymalizowany pod urządzenia mobilne. Okna czatu pojawiają się w trybie pełnoekranowym zamiast pływać obok tekstu, a opóźnienie zaznaczania jest usunięte dla bardziej natychmiastowej interakcji.

To zachowanie jest wbudowane i nie wymaga konfiguracji. Widżet automatycznie wykrywa rozmiar ekranu i dostosowuje się odpowiednio.

### Wygląd okna czatu

Okna czatu mają szerokość 410px na desktopie z 16px strzałką wskazującą na wyróżniony tekst. Okna pozycjonują się automatycznie w zależności od dostępnej przestrzeni widoku, używając klas pozycjonujących takich jak `to-right`, `to-left`, `to-top` i `to-bottom`.

Możesz dodać niestandardowy CSS, aby dostosować kolory, czcionki, odstępy lub inne właściwości wizualne tych okien. Okna czatu używają tej samej struktury komponentów co standardowy widżet FastComments, więc dziedziczą wszelkie globalne dostosowania, które zastosowałeś.

### Lokalizacja

Collab Chat obsługuje te same opcje lokalizacji co standardowy widżet FastComments. Ustaw opcję `locale`, aby wyświetlić tekst interfejsu w różnych językach:

[inline-code-attrs-start title = 'Ustaw lokalizację'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Hiszpański
});
[inline-code-end]

FastComments obsługuje dziesiątki języków. Ustawienie locale wpływa na cały tekst interfejsu użytkownika, w tym monity, przyciski i teksty zastępcze.

### Dziedziczone opcje dostosowywania

Ponieważ Collab Chat rozszerza standardowy widżet komentarzy, dziedziczy wszystkie opcje dostosowywania z podstawowego widżetu. Obejmuje to niestandardowe klasy CSS, niestandardowe tłumaczenia, dostosowanie avatarów, formatowanie dat i wiele innych.

Zobacz główną dokumentację dotyczącą dostosowywania FastComments, aby uzyskać pełną listę dostępnych opcji dostosowywania.

### Praca z niestandardowymi czcionkami

Jeśli Twoja strona używa niestandardowych czcionek, interfejs Collab Chat odziedziczy te czcionki z CSS Twojej strony. Może być konieczne utworzenie reguły dostosowywania widżetu i użycie `@import` dowolnych czcionek w niestandardowym CSS w tej regule, jeśli chcesz
aby pływające okno czatu korzystało z tych samych czcionek.