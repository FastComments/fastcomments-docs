### Швидкий старт

Почати роботу з Collab Chat просто. Вам потрібен скрипт FastComments Collab Chat, HTML-елемент із текстом, який ви хочете анотувати, та об'єкт конфігурації з вашим Tenant ID.

### Встановлення

Додайте скрипт Collab Chat на вашу сторінку:

[inline-code-attrs-start title = 'Завантаження скрипту Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Базова реалізація

Ось мінімальний приклад:

[inline-code-attrs-start title = 'Базова реалізація Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Замініть `'demo'` на ваш фактичний Tenant ID FastComments, якщо ви ще цього не зробили. Його можна знайти у вашій [панелі керування FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Як це працює

Після ініціалізації користувачі можуть виділяти будь-який текст у цільовому елементі. Після короткої затримки (3,5 секунди на настільних комп'ютерах) з'являється підказка, що дозволяє розпочати обговорення. Коли обговорення створено, на тексті з'являється візуальне виділення. Інші користувачі можуть навести курсор на виділення або клацнути по ньому, щоб переглянути обговорення та взяти в ньому участь. Усі обговорення синхронізуються в реальному часі для всіх відвідувачів.

### Жива демонстрація

Ви можете побачити Collab Chat у дії на нашій [сторінці живої демонстрації](https://fastcomments.com/product/collab-chat).

### Наступні кроки

Тепер, коли основи працюють, ви можете налаштувати зовнішній вигляд і поведінку за допомогою посібника з параметрів конфігурації. Ознайомтеся з посібником щодо поведінки виділення тексту, щоб зрозуміти, як працює виділення тексту. Дізнайтеся про стилізацію та підтримку темного режиму в посібнику з кастомізації. Для розширених інтеграцій вивчіть довідник API.

### Фронтенд-бібліотеки

Усі фронтенд-бібліотеки FastComments (react, vue, angular тощо) мають підтримку Collab Chat.