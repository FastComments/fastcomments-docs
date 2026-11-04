Najmanjša možna razširitev bi bila:

[inline-code-attrs-start title = 'Preprosta razširitev'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Za namen tega primera shranite to kot `my-extension.js` in poskrbite, da bo dostopna na `https://example.com/my-extension.min.js`.

Ta razširitev ne naredi ničesar, le ob nalaganju pridobi objekt razširitve, ustvarjen s strani osnovne knjižnice za komentarje.

Ta objekt `Extension` je singleton in ni v skupni rabi z drugimi razširitvami.

Nato, da naložimo našo razširitev, moramo o njej obvestiti gradnik za komentarje. Na primer:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Uporaba razširitve po meri'; code-example-end]

Za funkcionalne primere si oglejte naslednji razdelek.