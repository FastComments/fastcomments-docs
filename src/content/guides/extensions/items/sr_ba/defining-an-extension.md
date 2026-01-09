Najmanje moguće proširenje bi bilo:

[inline-code-attrs-start title = 'Jednostavno proširenje'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Za svrhe ovog primjera, sačuvajte ovo kao `my-extension.js`, i učinite ga dostupnim na `https://example.com/my-extension.min.js`.

Ovo proširenje ne radi ništa, osim što pri učitavanju dohvaća objekat proširenja koji je kreirala osnovna biblioteka komentara.

Ovaj `Extension` objekat je singleton i nije dijeljen sa bilo kojim drugim proširenjima.

Zatim, da bismo učitali naše proširenje, moramo obavijestiti widget za komentare o njemu. Na primjer:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Za funkcionalne primjere, pogledajte sljedeći odjeljak.