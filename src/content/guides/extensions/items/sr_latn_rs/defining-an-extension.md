Najmanje moguće proširenje bi bilo:

[inline-code-attrs-start title = 'Jednostavno proširenje'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Za potrebe ovog primera, sačuvajte ovo kao `my-extension.js`, i postavite ga na `https://example.com/my-extension.min.js`.

Ovo proširenje ništa ne radi, osim što pri učitavanju preuzima `Extension` objekat koji je kreirala osnovna biblioteka za komentare.

Ovaj `Extension` objekat je singleton i nije deljen sa nijednim drugim proširenjima.

Dalje, da bismo učitali naše proširenje, moramo obavestiti widget za komentare o njemu. Na primer:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Za funkcionalne primere, pogledajte sledeći odeljak.

---