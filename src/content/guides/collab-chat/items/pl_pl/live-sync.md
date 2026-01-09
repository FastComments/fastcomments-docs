### Aktualizacje w czasie rzeczywistym

Collab Chat używa połączeń WebSocket do synchronizacji wszystkich rozmów w czasie rzeczywistym między wszystkimi połączonymi użytkownikami. Gdy ktoś tworzy nową adnotację, dodaje komentarz lub usuwa dyskusję, wszyscy inni użytkownicy oglądający tę samą stronę widzą aktualizację natychmiast bez odświeżania.

### Jak działa synchronizacja przez WebSocket

Po zainicjowaniu Collab Chat widget nawiązuje połączenie WebSocket z serwerami FastComments. Połączenie to pozostaje otwarte przez czas trwania sesji użytkownika i nasłuchuje aktualizacji związanych z bieżącą stroną.

System WebSocket używa trzech typów komunikatów rozsyłanych (broadcast) dla Collab Chat. Zdarzenie `new-text-chat` uruchamia się, gdy ktoś tworzy nową adnotację na stronie. Zdarzenie `updated-text-chat` uruchamia się, gdy ktoś aktualizuje istniejącą konwersację. Zdarzenie `deleted-text-chat` uruchamia się, gdy ktoś usuwa adnotację.

### System Broadcast ID

Aby zapobiec efektowi echa, w którym użytkownicy widzą swoje własne działania przesyłane z powrotem, każda aktualizacja zawiera unikalny `broadcastId`. Gdy użytkownik tworzy lub aktualizuje adnotację, jego klient generuje UUID dla tej operacji. Gdy WebSocket rozsyła aktualizację z powrotem do wszystkich klientów, klient źródłowy ignoruje ją, ponieważ zgadza się z jego własnym `broadcastId`.

Zapewnia to płynne działanie: użytkownicy widzą swoje zmiany natychmiast w interfejsie bez oczekiwania na pełen obieg przez serwer, a jednocześnie wszyscy pozostali użytkownicy otrzymują aktualizację.

### Liczba użytkowników na żywo

Górny pasek pokazuje liczbę użytkowników aktualnie przeglądających stronę. Ta liczba aktualizuje się w czasie rzeczywistym, gdy użytkownicy dołączają i opuszczają stronę. Liczba użytkowników jest dostarczana przez to samo połączenie WebSocket i zwiększa/zmniejsza się automatycznie w oparciu o zdarzenia połączenia i rozłączenia.

### Odporność połączenia

Jeśli połączenie WebSocket zostanie przerwane z powodu problemów z siecią lub konserwacji serwera, widget automatycznie próbuje ponownie nawiązać połączenie. W czasie ponownego łączenia użytkownicy nadal mogą wchodzić w interakcje z istniejącymi adnotacjami, jednak nie będą widzieć aktualizacji w czasie rzeczywistym od innych użytkowników, dopóki połączenie nie zostanie przywrócone.

Po ponownym nawiązaniu połączenia widget ponownie synchronizuje stan, aby upewnić się, że nie przegapiono żadnych aktualizacji. Dzieje się to w przejrzysty sposób, bez konieczności ingerencji użytkownika.

### Uwagi dotyczące przepustowości

Komunikaty WebSocket są lekkie i zawierają tylko niezbędne informacje potrzebne do synchronizacji stanu. Utworzenie nowej adnotacji zazwyczaj zużywa mniej niż 1KB przepustowości. System zawiera również inteligentne grupowanie komunikatów (batching), aby zmniejszyć częstotliwość wiadomości w okresach dużej aktywności.

Twoje metryki użycia w panelu FastComments śledzą `pubSubMessageCount` i `pubSubBandwidth`, dzięki czemu możesz monitorować aktywność synchronizacji w czasie rzeczywistym na swoich witrynach.

### Synchronizacja między zakładkami

Jeśli użytkownik ma tę samą stronę otwartą w wielu kartach przeglądarki, aktualizacje w jednej karcie pojawiają się natychmiast w pozostałych kartach. Działa to przez ten sam mechanizm synchronizacji WebSocket i nie wymaga dodatkowej konfiguracji.

### Bezpieczeństwo

Komunikaty WebSocket są przesyłane przez bezpieczne połączenia (WSS) i zawierają walidację najemcy (tenant validation), aby zapewnić, że użytkownicy otrzymują tylko aktualizacje do konwersacji, do których mają uprawnienia. Serwer weryfikuje wszystkie operacje przed ich rozesłaniem, aby zapobiec nieautoryzowanemu dostępowi lub manipulacji.