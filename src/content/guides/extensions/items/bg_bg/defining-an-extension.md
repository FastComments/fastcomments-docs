Най-малкото възможно разширение би изглеждало така:

[inline-code-attrs-start title = 'Просто разширение'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

За целите на този пример, запазете това като `my-extension.js`, и го направете достъпно на `https://example.com/my-extension.min.js`.

Това разширение не прави нищо, освен че при зареждане извлича обекта на разширението, създаден от основната библиотека за коментари.

Този `Extension` обект е единствен екземпляр и не се споделя с други разширения.

След това, за да заредим нашето разширение, трябва да уведомим коментарния уиджет за него. Например:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

За функционални примери вижте следващия раздел.

---