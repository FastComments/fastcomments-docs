[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments позволява на потребителите да качват изображения. Когато потребителят кликне върху това изображение, FastComments по подразбиране,
ще отвори нов раздел, за да покаже изображението в пълен размер. Задаването на този флаг на true ще деактивира това поведение:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Ако не възнамерявате сами да обработвате кликването върху изображението (вж. [onImageClicked](#callbacks)), препоръчваме това да се комбинира с някакво стилизиране
за да се премахне впечатлението, че изображението може да бъде кликнато.