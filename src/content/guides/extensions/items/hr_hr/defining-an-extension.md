Najmanje moguće proširenje bilo bi:

[inline-code-attrs-start title = 'Jednostavno proširenje'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

U svrhu ovog primjera, spremite ovo kao `my-extension.js`, i učinite ga dostupnim na `https://example.com/my-extension.min.js`.

Ovo proširenje ne radi ništa; pri učitavanju jedino dohvaća objekt proširenja koji je stvorila osnovna biblioteka komentara.

Ovaj `Extension` objekt je singleton i nije dijeljen s nijednim drugim proširenjem.

Sljedeće, da bismo učitali naše proširenje, moramo o tome obavijestiti widget za komentare. Na primjer:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Za funkcionalne primjere, pogledajte sljedeći odjeljak.

---