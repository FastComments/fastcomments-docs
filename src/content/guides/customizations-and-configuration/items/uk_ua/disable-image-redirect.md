[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments дозволяє користувачам завантажувати зображення. Коли користувач натискає на це зображення, FastComments за замовчуванням
відкриває нову вкладку, щоб показати зображення у повному розмірі. Встановлення цього прапорця в true вимикає цю поведінку:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Якщо ви не плануєте самостійно перехоплювати натиск на зображення (див. [onImageClicked](#callbacks)), ми рекомендуємо поєднати це з певними стилями
щоб прибрати вигляд того, що зображення можна натиснути.