### Pozycjonowanie oparte na procentach

Image Chat używa współrzędnych procentowych zamiast współrzędnych pikselowych do pozycjonowania znaczników czatu na obrazach. Gdy użytkownik kliknie obraz, widżet konwertuje współrzędne pikselowe kliknięcia na procenty szerokości i wysokości obrazu. To podejście zapewnia, że znaczniki pozostają we właściwym miejscu niezależnie od tego, jak obraz jest wyświetlany.

Na przykład, jeśli użytkownik kliknie 250 pikseli od lewej krawędzi obrazu o szerokości 1000px, widżet zapisuje to jako 25% od lewej. Gdy inny użytkownik wyświetli ten sam obraz o szerokości 500px na urządzeniu mobilnym, znacznik pojawi się 125 pikseli od lewej, co nadal stanowi 25% szerokości.

### Korzyści dla responsywnych układów

Ten system procentowy sprawia, że Image Chat działa płynnie na wszystkich rozmiarach i orientacjach urządzeń. Twoje obrazy mogą być wyświetlane w różnych rozmiarach w zależności od szerokości ekranu, reguł CSS lub ograniczeń kontenera, ale znaczniki zawsze będą poprawnie wyrównane z zamierzonymi lokalizacjami.

Użytkownicy na komputerach stacjonarnych z dużymi monitorami widzą znaczniki w tych samych pozycjach względnych co użytkownicy smartfonów z małymi ekranami. Znaczniki skalują się proporcjonalnie do samego obrazu.

### Skalowanie obrazu i proporcje

Dopóki Twój obraz zachowuje swoje proporcje podczas skalowania (co jest domyślnym zachowaniem przeglądarki), znaczniki procentowe pozostaną idealnie wyrównane. Widżet zakłada, że obrazy skalują się proporcjonalnie i oblicza pozycje na podstawie tego założenia.

Jeśli zastosujesz CSS, który zniekształca proporcje obrazu (np. używając `object-fit: cover` z określonymi wymiarami), pozycje znaczników mogą nie być poprawnie wyrównane. Dla najlepszych rezultatów pozwól obrazom skalować się naturalnie lub użyj `object-fit: contain`, aby zachować proporcje.

### Rozmiar kwadratów czatu

Wizualny rozmiar znaczników czatu również jest oparty na procentach. Opcja konfiguracji `chatSquarePercentage` ma domyślnie wartość 5%, co oznacza, że każdy kwadrat stanowi 5% szerokości obrazu. Zapewnia to spójną wagę wizualną w różnych rozmiarach obrazów.

Na obrazie o szerokości 1000px z domyślnym ustawieniem 5% znaczniki mają 50px kwadratu. Na obrazie o szerokości 500px te same znaczniki mają 25px kwadratu. Pozostają proporcjonalne do rozmiaru obrazu.

### Zachowanie na urządzeniach mobilnych

Na ekranach o szerokości poniżej 768px Image Chat przełącza się na układ zoptymalizowany pod urządzenia mobilne. Okna czatu otwierają się na pełnym ekranie zamiast unosić się obok znacznika. Zapewnia to lepszą użyteczność na małych ekranach, gdzie pływające okna zasłaniałyby zbyt dużą część obrazu.

Same znaczniki pozostają widoczne i klikalne na swoich pozycjach procentowych. Użytkownicy nadal mogą zobaczyć, gdzie znajdują się wszystkie dyskusje, i dotknąć znaczników, aby otworzyć pełnoekranowy interfejs czatu.

### Dynamiczne ładowanie obrazów

System procentowy działa poprawnie nawet wtedy, gdy obrazy ładują się asynchronicznie lub zmieniają rozmiar po załadowaniu strony. Widżet monitoruje element obrazu i przelicza pozycje znaczników za każdym razem, gdy wymiary obrazu się zmienią.

Jeśli stosujesz leniwe ładowanie obrazów lub implementujesz responsywne obrazy o różnych rozmiarach w różnych breakpointach, znaczniki automatycznie dostosują się po zmianie rozmiaru obrazu.

### Spójność między urządzeniami

Ponieważ współrzędne są przechowywane jako procenty w bazie danych, dyskusja utworzona na komputerze stacjonarnym pojawia się w dokładnie tej samej pozycji względnej podczas wyświetlania na tablecie lub telefonie. Użytkownicy mogą współpracować między urządzeniami bez żadnych niespójności w pozycjonowaniu.

Działa to dwukierunkowo. Dyskusja utworzona przez dotknięcie konkretnego miejsca na urządzeniu mobilnym pojawia się w tej samej pozycji względnej podczas wyświetlania na dużym monitorze komputera stacjonarnego.

### Uwagi dotyczące viewportu

Widżet oblicza procenty względem samego elementu obrazu, a nie viewportu. Oznacza to, że przewijanie strony lub zmiana rozmiaru okna przeglądarki nie wpływa na pozycje znaczników. Znaczniki pozostają zakotwiczone w swoich lokalizacjach na obrazie niezależnie od zmian viewportu.

### Przyszłościowa zawartość

Podejście oparte na procentach sprawia, że Twoje dyskusje na obrazach są odporne na zmiany układu, designu lub ekosystemu urządzeń. Gdy pojawią się nowe rozmiary ekranów i urządzenia, istniejące dyskusje będą nadal wyświetlać się poprawnie bez konieczności jakichkolwiek aktualizacji lub migracji.