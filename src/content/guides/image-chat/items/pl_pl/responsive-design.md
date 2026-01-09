### Percentage-Based Positioning

Image Chat używa współrzędnych opartych na procentach zamiast współrzędnych w pikselach do pozycjonowania znaczników czatu na obrazach. Gdy użytkownik kliknie obraz, widget konwertuje współrzędne pikselowe kliknięcia na procenty szerokości i wysokości obrazu. Takie podejście zapewnia, że znaczniki pozostają na właściwym miejscu niezależnie od sposobu wyświetlania obrazu.

Na przykład, jeśli użytkownik kliknie 250 pikseli od lewej krawędzi obrazu o szerokości 1000px, widget zapisuje to jako 25% od lewej. Gdy inny użytkownik zobaczy ten sam obraz o szerokości 500px na urządzeniu mobilnym, znacznik pojawi się 125 pikseli od lewej, co wciąż stanowi 25% szerokości.

### Benefits for Responsive Layouts

Ten system oparty na procentach sprawia, że Image Chat działa bezproblemowo na wszystkich rozmiarach urządzeń i orientacjach. Twoje obrazy mogą być wyświetlane w różnych rozmiarach w zależności od szerokości ekranu, reguł CSS lub ograniczeń kontenera, ale znaczniki zawsze wyrównują się prawidłowo z zamierzonymi miejscami.

Użytkownicy na komputerach stacjonarnych z dużymi monitorami widzą znaczniki w tych samych względnych pozycjach co użytkownicy na smartfonach z małymi ekranami. Znaczniki skalują się proporcjonalnie wraz z obrazem.

### Image Scaling and Aspect Ratio

O ile obraz zachowuje swój współczynnik proporcji podczas skalowania (co jest domyślnym zachowaniem przeglądarki), znaczniki oparte na procentach pozostaną idealnie wyrównane. Widget zakłada, że obrazy skalują się proporcjonalnie i oblicza pozycje na tej podstawie.

Jeśli zastosujesz CSS, które zniekształca proporcje obrazu (na przykład używając `object-fit: cover` z określonymi wymiarami), pozycje znaczników mogą nie być wyrównane prawidłowo. Dla najlepszych rezultatów pozwól obrazom skalować się naturalnie lub użyj `object-fit: contain`, aby zachować współczynnik proporcji.

### Chat Square Sizing

Wizualny rozmiar znaczników czatu jest również oparty na procentach. Opcja konfiguracji `chatSquarePercentage` ma domyślną wartość 5%, co oznacza, że każdy kwadrat ma 5% szerokości obrazu. Zapewnia to spójny wygląd wizualny niezależnie od rozmiaru obrazu.

Na obrazie o szerokości 1000px przy domyślnym ustawieniu 5% znaczniki mają 50px kwadrat. Na obrazie o szerokości 500px te same znaczniki mają 25px kwadrat. Pozostają proporcjonalne do rozmiaru obrazu.

### Mobile Behavior

Na ekranach o szerokości poniżej 768px Image Chat przełącza się na układ zoptymalizowany pod kątem urządzeń mobilnych. Okna czatu otwierają się na pełnym ekranie zamiast unosić się obok znacznika. Zapewnia to lepszą użyteczność na małych ekranach, gdzie unoszące się okna zasłaniałyby zbyt dużą część obrazu.

Same znaczniki pozostają widoczne i klikalne w swoich pozycjach opartych na procentach. Użytkownicy nadal mogą zobaczyć, gdzie znajdują się wszystkie dyskusje i stuknąć znaczniki, aby otworzyć interfejs czatu na pełnym ekranie.

### Dynamic Image Loading

System oparty na procentach działa poprawnie nawet wtedy, gdy obrazy ładują się asynchronicznie lub zmieniają rozmiar po załadowaniu strony. Widget monitoruje element obrazu i ponownie oblicza pozycje znaczników za każdym razem, gdy zmieniają się wymiary obrazu.

Jeśli stosujesz lazy-loading obrazów lub implementujesz obrazy responsywne o różnych rozmiarach na różnych punktach przerwania, znaczniki automatycznie dostosowują się, gdy zmienia się rozmiar obrazu.

### Cross-Device Consistency

Ponieważ współrzędne są przechowywane w bazie danych jako procenty, dyskusja utworzona na komputerze stacjonarnym pojawia się w dokładnie tym samym względnym miejscu po wyświetleniu na tablecie lub telefonie. Użytkownicy mogą współpracować na różnych urządzeniach bez problemów z pozycjonowaniem.

Działa to w obie strony. Dyskusja utworzona przez stuknięcie konkretnego miejsca na urządzeniu mobilnym pojawia się w tej samej względnej pozycji po wyświetleniu na dużym monitorze stacjonarnym.

### Viewport Considerations

Widget oblicza procenty względem samego elementu obrazu, a nie widoku (viewport). Oznacza to, że przewijanie strony lub zmiana rozmiaru okna przeglądarki nie wpływa na pozycje znaczników. Znaczniki pozostają zakotwiczone w swoich pozycjach na obrazie niezależnie od zmian widoku.

### Future-Proofing Content

Podejście oparte na procentach sprawia, że Twoje dyskusje na obrazach są odporne na zmiany układu, projektu czy ekosystemu urządzeń. W miarę pojawiania się nowych rozmiarów ekranu i urządzeń istniejące dyskusje będą nadal wyświetlane poprawnie bez potrzeby aktualizacji lub migracji.