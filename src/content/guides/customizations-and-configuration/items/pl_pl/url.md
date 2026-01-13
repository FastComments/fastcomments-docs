[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Kiedy wysyłasz powiadomienia e-mail, lub renderujesz komentarze w interfejsach użytkownika, takich jak strona moderacji, przydatne jest umożliwienie linkowania
z komentarza do strony, na której się znajduje.

Jeśli identyfikator URL nie zawsze jest rzeczywistym identyfikatorem, musimy przechowywać URL gdzie indziej. Do tego służy właściwość "url", zdefiniowana w następujący sposób.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Typowym przypadkiem użycia jest powiązanie wątku komentarzy z identyfikatorem, takim jak artykuł, a następnie odwołanie się do konkretnej strony, na przykład:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL nie jest oczyszczany z typowych parametrów marketingowych. Domyślnie zapisywany z komentarzem jest dokładnie bieżący URL strony.