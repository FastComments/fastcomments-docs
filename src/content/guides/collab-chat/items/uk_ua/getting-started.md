### Quick Start

Почати роботу з Collab Chat просто. Вам потрібен скрипт FastComments Collab Chat, HTML-елемент, який містить текст, що його потрібно анотувати, та об'єкт конфігурації з вашим Tenant ID.

### Installation

Додайте скрипт Collab Chat на вашу сторінку:

[inline-code-attrs-start title = 'Завантаження скрипта Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Basic Implementation

Ось мінімальний приклад:

[inline-code-attrs-start title = 'Базова реалізація Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Ваш контейнер вмісту -->
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

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### How It Works

Після ініціалізації користувачі можуть виділити будь-який текст у цільовому елементі. Після короткої затримки (3.5 секунди на десктопі) з'являється підказка, яка дозволяє почати обговорення. Коли створюється обговорення, на тексті з'являється візуальне підсвічування. Інші користувачі можуть навести курсор або натиснути на підсвічування, щоб переглянути та взяти участь в обговоренні. Усі обговорення синхронізуються в режимі реального часу між усіма відвідувачами.

### Live Demo

Ви можете побачити Collab Chat в дії на нашій [сторінці демонстрації](https://fastcomments.com/product/collab-chat).

### Next Steps

Тепер, коли базова функціональність працює, ви можете налаштувати зовнішній вигляд і поведінку в керівництві по параметрах конфігурації. Ознайомтеся з керівництвом з поведінки виділення тексту, щоб зрозуміти, як працює виділення тексту. Дізнайтеся про стилізацію та підтримку темної теми в керівництві з налаштування. Для розширених інтеграцій вивчіть API Reference.

### Frontend Libraries

Усі фронтенд-бібліотеки FastComments (react, vue, angular тощо) мають Collab Chat.