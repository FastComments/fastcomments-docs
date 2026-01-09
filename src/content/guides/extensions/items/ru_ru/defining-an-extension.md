Наименьшее возможное расширение будет выглядеть так:

[inline-code-attrs-start title = 'Простое расширение'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Для этого примера сохраните это как `my-extension.js` и разместите по адресу `https://example.com/my-extension.min.js`.

Это расширение ничего не делает, за исключением того, что при загрузке оно получает объект расширения, созданный основной библиотекой комментариев.

Этот `Extension` объект является синглтоном и не разделяется с другими расширениями.

Далее, чтобы загрузить наше расширение, нужно сообщить виджету комментариев о нём. Например:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Для практических примеров смотрите следующий раздел.

---