Найменше можливе розширення виглядає так:

[inline-code-attrs-start title = 'Просте розширення'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Для цього прикладу збережіть цей файл як `my-extension.js` і зробіть його доступним за адресою `https://example.com/my-extension.min.js`.

Це розширення нічого не робить, окрім того, що при завантаженні воно отримує об'єкт розширення, створений основною бібліотекою коментарів.

This `Extension` object is a singleton and is not shared with any other extensions.

Далі, щоб завантажити наше розширення, ми повинні повідомити віджет коментарів про нього. Наприклад:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Для функціональних прикладів див. наступний розділ.

---