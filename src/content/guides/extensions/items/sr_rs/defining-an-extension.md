Најмање могуће проширење изгледало би овако:

[inline-code-attrs-start title = 'Једноставно проширење'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Заради овог примера, сачувајте ово као `my-extension.js`, и учините га доступним на `https://example.com/my-extension.min.js`.

Ово проширење не ради ништа посебно, осим што при учитавању преузима објекат проширења који креира основна библиотека коментара.

Овај објекат `Extension` је јединичан (singleton) и није дељен са другим проширењима.

Даље, да бисмо учитали наше проширење, морамо обавестити видгет за коментаре о њему. На пример:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

За функционалне примере, погледајте следећи одељак.

---