Najmanjša možna razširitev bi bila:

[inline-code-attrs-start title = 'Preprosta razširitev'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

V ta primer shranite datoteko kot `my-extension.js`, in jo naredite dostopno na `https://example.com/my-extension.min.js`.

Ta razširitev ne počne ničesar, ob nalaganju pa pridobi objekt razširitve, ki ga ustvari osnovna knjižnica komentarjev.

This `Extension` object is a singleton and is not shared with any other extensions.

Da naložimo našo razširitev, moramo pripomočku za komentarje povedati za njo. Na primer:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Za funkcionalne primere glejte naslednji razdelek.

---