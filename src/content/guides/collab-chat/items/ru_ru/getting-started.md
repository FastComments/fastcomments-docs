### Быстрый старт

Начать работу с Collab Chat просто. Вам понадобятся скрипт FastComments Collab Chat, HTML-элемент с текстом, который вы хотите аннотировать, и объект конфигурации с вашим Tenant ID.

### Установка

Добавьте скрипт Collab Chat на вашу страницу:

[inline-code-attrs-start title = 'Загрузка скрипта Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Базовая реализация

Вот минимальный пример:

[inline-code-attrs-start title = 'Базовая реализация Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Ваш контейнер с контентом -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Загрузите скрипт Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Инициализируйте Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Замените `'demo'` на ваш реальный FastComments Tenant ID, если ещё не сделали этого. Его можно найти в вашей [панели управления FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Как это работает

После инициализации пользователи могут выделять любой текст внутри целевого элемента. После небольшой задержки (3,5 секунды на десктопе) появляется подсказка, позволяющая начать обсуждение. Когда обсуждение создано, на тексте появляется визуальное выделение. Другие пользователи могут навести курсор на выделение или кликнуть по нему, чтобы просмотреть обсуждение и принять в нём участие. Все обсуждения синхронизируются в реальном времени у всех посетителей.

### Живое демо

Вы можете увидеть Collab Chat в действии на нашей [странице живого демо](https://fastcomments.com/product/collab-chat).

### Следующие шаги

Теперь, когда основы работают, вы можете настроить внешний вид и поведение в руководстве по параметрам конфигурации (Configuration Options). Ознакомьтесь с руководством по поведению выделения текста (Text Selection Behavior), чтобы понять, как работает выделение текста. Узнайте о стилизации и поддержке тёмной темы в руководстве по кастомизации (Customization). Для продвинутых интеграций изучите справочник API (API Reference).

### Фронтенд-библиотеки

Все фронтенд-библиотеки FastComments (react, vue, angular и т.д.) поддерживают Collab Chat.