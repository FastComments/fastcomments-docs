[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Domyślnie widżet FastComments automatycznie dopasowuje swoją wysokość, aby zmieścić wszystkie widoczne komentarze. Paginacja jest realizowana za pomocą przycisku "Pokaż następne" na końcu bieżącej strony, ponieważ uznaliśmy, że taka interakcja jest najbardziej naturalna dla większości użytkowników.

Jednak w niektórych przypadkach preferowane jest przewijanie nieskończone. Na przykład używamy tej funkcji w naszym produkcie Stream Chat.

Możemy ukryć przyciski "Pokaż następne" i przełączyć na przewijanie nieskończone ustawiając flagę **enableInfiniteScrolling** na true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Wymaga to również dodania niestandardowego CSS. Dodaj niestandardowy CSS dla selektora `.comments`, aby włączyć przewijanie, na przykład:

[inline-code-attrs-start title = 'Włączenie przewijania nieskończonego'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Pełny, działający przykład wyglądałby następująco:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

W powyższym przykładzie używamy właściwości `customCSS`, jednak zalecane jest użycie interfejsu konfiguracji widżetu ze względów wydajnościowych. [Zobacz dokumentację niestandardowego CSS.](/guide-customizations-and-configuration.html#custom-css)