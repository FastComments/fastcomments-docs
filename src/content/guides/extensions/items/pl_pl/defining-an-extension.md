Najmniejsze możliwe rozszerzenie wyglądałoby tak:

[inline-code-attrs-start title = 'Proste rozszerzenie'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Na potrzeby tego przykładu zapisz to jako `my-extension.js` i udostępnij pod adresem `https://example.com/my-extension.min.js`.

To rozszerzenie nic nie robi, poza tym że przy załadowaniu pobiera obiekt rozszerzenia utworzony przez główną bibliotekę komentarzy.

Ten obiekt `Extension` jest singletonem i nie jest współdzielony z żadnymi innymi rozszerzeniami.

Następnie, aby załadować nasze rozszerzenie, musimy poinformować o nim widżet komentarzy. Na przykład:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Użycie niestandardowego rozszerzenia'; code-example-end]

Przykłady funkcjonalne znajdziesz w następnej sekcji.