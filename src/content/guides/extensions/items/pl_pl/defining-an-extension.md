Najmniejsze możliwe rozszerzenie wyglądałoby następująco:

[inline-code-attrs-start title = 'Proste rozszerzenie'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Na potrzeby tego przykładu zapisz to jako `my-extension.js` i udostępnij pod adresem `https://example.com/my-extension.min.js`.

To rozszerzenie nic nie robi, poza tym przy wczytaniu pobiera obiekt rozszerzenia utworzony przez główną bibliotekę komentarzy.

This `Extension` object is a singleton and is not shared with any other extensions.

Aby załadować nasze rozszerzenie, musimy poinformować o tym widżet komentarzy. Na przykład:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Przykłady funkcjonalne znajdują się w następnej sekcji.

---