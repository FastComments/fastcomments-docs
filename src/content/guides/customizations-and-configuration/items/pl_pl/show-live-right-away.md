[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Domyślnie komentowanie na żywo jest włączone. Oznacza to, że jeśli jakiekolwiek komentarze zostaną dodane, usunięte, edytowane lub przypięte, zmiany powinny pojawić się
dla wszystkich użytkowników przeglądających wątek komentarzy w tym samym czasie.

Jednak domyślnie nowe komentarze pojawią się pod dynamicznie wyświetlanym przyciskiem z tekstem podobnym do "Pokaż 2 nowe komentarze".

Jeżeli nowe komentarze są odpowiedziami bezpośrednio do strony, przycisk pojawi się u góry wątku komentarzy. Jeśli są odpowiedziami na konkretny komentarz, 
przycisk pojawi się pod tym komentarzem.

Ma to zapobiec ciągłej zmianie rozmiaru strony dla użytkownika, co może powodować frustrację przy próbie chwycenia paska przewijania.

W niektórych przypadkach użycia, takich jak licytacje na żywo, lub wydarzenia online, takie zachowanie nie jest pożądane - możesz chcieć, aby widżet komentarzy był
bardziej jak pole "chat", gdzie nowe komentarze "pokazują się od razu".

Stąd nazwa flagi, która włącza tę funkcję: **showLiveRightAway**.

Możemy ją włączyć w następujący sposób:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Można to dostosować bez kodu, na stronie personalizacji widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]