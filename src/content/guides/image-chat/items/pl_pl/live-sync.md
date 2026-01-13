### Aktualizacje w czasie rzeczywistym

Image Chat używa połączeń WebSocket do synchronizacji wszystkich konwersacji w czasie rzeczywistym między wszystkimi podłączonymi użytkownikami. Gdy ktoś tworzy nowy znacznik, dodaje komentarz lub usuwa dyskusję, wszyscy inni użytkownicy oglądający ten sam obraz widzą aktualizację natychmiast, bez odświeżania.

### Jak działa synchronizacja przez WebSocket

Gdy inicjujesz Image Chat, widget nawiązuje połączenie WebSocket z serwerami FastComments. Połączenie to pozostaje otwarte przez cały czas trwania sesji użytkownika i nasłuchuje aktualizacji związanych z bieżącym obrazem.

System WebSocket używa trzech typów komunikatów rozgłoszeniowych dla Image Chat. Zdarzenie `new-image-chat` wywoływane jest, gdy ktoś tworzy nowy znacznik na obrazie. Zdarzenie `image-chat-updated` wywoływane jest, gdy ktoś aktualizuje istniejącą konwersację. Zdarzenie `deleted-image-chat` wywoływane jest, gdy ktoś usuwa znacznik.

### System identyfikatorów broadcast

Aby zapobiec efektom echa, w których użytkownicy widzą własne działania rozgłaszane z powrotem do nich, każda aktualizacja zawiera unikalny `broadcastId`. Gdy użytkownik tworzy lub aktualizuje znacznik, jego klient generuje UUID dla tej operacji. Gdy WebSocket rozgłasza aktualizację z powrotem do wszystkich klientów, klient źródłowy ignoruje tę aktualizację, ponieważ pasuje ona do jego własnego `broadcastId`.

Zapewnia to płynną interakcję, w której użytkownicy widzą swoje zmiany natychmiast w interfejsie użytkownika bez oczekiwania na pełną podróż przez serwer, jednocześnie zapewniając, że wszyscy inni użytkownicy otrzymają aktualizację.

### Odporność połączenia

Jeśli połączenie WebSocket zostanie przerwane z powodu problemów sieciowych lub konserwacji serwera, widget automatycznie próbuje ponownie nawiązać połączenie. W okresie ponownego łączenia użytkownicy nadal mogą wchodzić w interakcje z istniejącymi znacznikami, ale nie będą widzieć aktualizacji w czasie rzeczywistym od innych użytkowników, dopóki połączenie nie zostanie przywrócone.

Po ponownym połączeniu widget ponownie synchronizuje stan, aby upewnić się, że nie pominął żadnych aktualizacji. Dzieje się to w sposób przezroczysty, bez potrzeby interwencji użytkownika.

### Uwagi dotyczące przepustowości

Komunikaty WebSocket są lekkie i zawierają tylko niezbędne informacje potrzebne do synchronizacji stanu. Utworzenie nowego znacznika zwykle zużywa mniej niż 1KB przepustowości. System zawiera również inteligentne grupowanie, aby zmniejszyć częstotliwość komunikatów w okresach dużej aktywności.

Twoje metryki użycia w panelu FastComments śledzą `pubSubMessageCount` i `pubSubBandwidth`, dzięki czemu możesz monitorować aktywność synchronizacji w czasie rzeczywistym na swoich witrynach.

### Synchronizacja między kartami

Jeśli użytkownik ma tę samą stronę otwartą w wielu kartach przeglądarki, aktualizacje w jednej karcie pojawiają się natychmiast w pozostałych kartach. Działa to przez ten sam mechanizm synchronizacji WebSocket i nie wymaga żadnej dodatkowej konfiguracji.

Użytkownicy mogą mieć Twoją witrynę otwartą na wielu urządzeniach jednocześnie, i wszystkie będą pozostawać zsynchronizowane. Znacznik utworzony na komputerze stacjonarnym pojawi się natychmiast na tablecie użytkownika, jeśli oba urządzenia wyświetlają ten sam obraz.

### Bezpieczeństwo

Komunikaty WebSocket są przesyłane przez bezpieczne połączenia (WSS) i zawierają walidację najemcy, aby upewnić się, że użytkownicy otrzymują tylko aktualizacje do rozmów, do których są uprawnieni. Serwer weryfikuje wszystkie operacje przed ich rozgłoszeniem, aby zapobiec nieautoryzowanemu dostępowi lub manipulacji.

### Zachowanie w trybie offline

Gdy użytkownicy są całkowicie offline, nadal mogą przeglądać istniejące znaczniki, ale nie mogą tworzyć nowych ani zobaczyć aktualizacji od innych. Widget wykrywa stan offline i wyświetla odpowiednie komunikaty.

Jeśli użytkownik spróbuje utworzyć znacznik będąc offline, a następnie wróci online, operacja zakończy się niepowodzeniem zamiast być kolejkowana, co zapewnia spójność danych. Użytkownicy powinni ponowić próbę operacji po przywróceniu połączenia.

### Wpływ na wydajność

Połączenie WebSocket ma minimalny wpływ na wydajność. Połączenie pozostaje bezczynne, gdy nie występują żadne aktualizacje i przetwarza komunikaty tylko wtedy, gdy pojawia się aktywność. Dla typowego obrazu przy umiarkowanej aktywności znaczników, WebSocket zużywa mniej CPU niż samo renderowanie obrazu.

Dla stron z setkami jednoczesnych użytkowników i wysoką aktywnością tworzenia znaczników, system skaluje się horyzontalnie, aby utrzymać wydajność bez wpływu na pojedyncze połączenia klientów.

### Zastosowania w pracy zespołowej

Synchronizacja w czasie rzeczywistym czyni Image Chat szczególnie przydatnym w przepływach pracy wymagających współpracy. Zespoły projektowe mogą wspólnie przeglądać makiety, a wszyscy widzą rozmieszczenie znaczników w czasie rzeczywistym. Zespoły wsparcia klienta mogą wspólnie adnotować zrzuty ekranu, aby zidentyfikować problemy. Grupy edukacyjne mogą omawiać diagramy, a wszyscy uczestnicy widzą znaczniki dodawane przez innych w miarę ich tworzenia.

Natychmiastowa informacja zwrotna tworzy bardziej angażujące i produktywne doświadczenie współpracy w porównaniu z tradycyjnymi systemami komentarzy, gdzie użytkownicy muszą odświeżyć stronę, aby zobaczyć aktualizacje.