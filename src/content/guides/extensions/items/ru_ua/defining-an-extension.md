---
Самое маленькое возможное расширение будет:

[inline-code-attrs-start title = 'Простое расширение'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

В целях этого примера сохраните это как `my-extension.js`, и сделайте его доступным по `https://example.com/my-extension.min.js`.

Это расширение ничего не делает, кроме того что при загрузке получает объект расширения, созданный основной библиотекой комментариев.

Объект `Extension` является синглтоном и не используется совместно с другими расширениями.

Далее, чтобы загрузить наше расширение, мы должны сообщить о нём виджету комментариев. Например:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Для практических примеров см. следующий раздел.

---