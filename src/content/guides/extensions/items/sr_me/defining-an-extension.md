Најмање могуће проширење би било:

[inline-code-attrs-start title = 'Једноставно проширење'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

У овом примеру, сачувајте ово као `my-extension.js`, и учините га доступним на `https://example.com/my-extension.min.js`.

Ово проширење не ради ништа, осим што при учитавању преузима објекат проширења који је креирала основна библиотека коментара.

Овај `Extension` објекат је синглтон и није дељен са ниједним другим проширењем.

Затим, да бисмо учитали наше проширење, морамо обавестити видџет за коментаре о њему. На пример:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

За функционалне примјере, погледајте следећи одељак.