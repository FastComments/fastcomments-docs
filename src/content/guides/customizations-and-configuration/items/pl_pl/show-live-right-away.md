[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Domyślnie komentarze na żywo są włączone. Oznacza to, że jeśli jakiekolwiek komentarze zostaną dodane, usunięte, edytowane lub przypięte, zmiany powinny pojawić się u wszystkich użytkowników przeglądających wątek komentarzy w tym samym czasie.

Jednak domyślnie nowe komentarze będą wyświetlane pod dynamicznie pokazywanym przyciskiem z tekstem podobnym do "Show 2 New Comments".

Jeśli nowe komentarze są odpowiedziami bezpośrednio do strony, przycisk pojawi się na górze wątku komentarzy. Jeśli są odpowiedziami do konkretnego komentarza, przycisk pojawi się pod tym komentarzem.

Ma to na celu zapobieżenie ciągłej zmianie rozmiaru strony dla użytkownika, co mogłoby powodować frustrację przy próbie chwycenia paska przewijania.

W niektórych przypadkach, takich jak licytacje na żywo czy wydarzenia online, takie zachowanie nie jest pożądane – możesz chcieć, aby widżet komentarzy działał bardziej jak "chat" box, gdzie nowe komentarze "show right away".

Stąd nazwa flagi, która włącza tę funkcję: **showLiveRightAway**.

Możemy ją włączyć w następujący sposób:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Pokaż komentarze na żywo od razu'; code-example-end]

Można to dostosować bez kodu, na stronie personalizacji widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Ustawienie ukrywania komentarzy na żywo włączone, więc nowe komentarze pojawiają się od razu zamiast za przyciskiem'; title='Pokaż komentarze na żywo od razu' app-screenshot-end]