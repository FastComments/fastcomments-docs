Найпростіше можливе розширення виглядатиме так:

[inline-code-attrs-start title = 'Просте розширення'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Для цілей цього прикладу збережіть його як `my-extension.js` і зробіть доступним за адресою `https://example.com/my-extension.min.js`.

Це розширення нічого не робить, окрім того, що під час завантаження воно отримує об'єкт розширення, створений основною бібліотекою коментарів.

Цей об'єкт `Extension` є синглтоном і не надається спільно жодним іншим розширенням.

Далі, щоб завантажити наше розширення, ми маємо повідомити про нього віджет коментарів. Наприклад:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Використання власного розширення'; code-example-end]

Функціональні приклади дивіться в наступному розділі.