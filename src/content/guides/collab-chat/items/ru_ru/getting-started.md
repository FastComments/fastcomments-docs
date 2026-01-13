### Быстрый старт

Начать работу с Collab Chat просто. Вам нужен скрипт FastComments Collab Chat, HTML-элемент, содержащий текст, который вы хотите аннотировать, и объект конфигурации с вашим Tenant ID.

### Установка

Добавьте скрипт Collab Chat на вашу страницу:

[inline-code-attrs-start title = 'Загрузка скрипта Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Базовая реализация

Ниже приведён минимальный пример:

[inline-code-attrs-start title = 'Базовая реализация Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Контейнер с содержимым -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Инициализировать Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [панель управления FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Как это работает

После инициализации пользователи могут выделять любой текст внутри целевого элемента. После небольшой задержки (3,5 секунды на десктопе) появляется подсказка, позволяющая начать обсуждение. Когда обсуждение создаётся, на тексте появляется визуальное выделение. Другие пользователи могут навести курсор или кликнуть по выделению, чтобы просмотреть и принять участие в обсуждении. Все обсуждения синхронизируются в реальном времени между всеми посетителями.

### Живая демонстрация

Вы можете увидеть Collab Chat в действии на нашей [странице живой демонстрации](https://fastcomments.com/product/collab-chat).

### Следующие шаги

Теперь, когда базовая функциональность работает, вы можете настроить внешний вид и поведение в руководстве по параметрам конфигурации. Ознакомьтесь с руководством по поведению при выделении текста, чтобы понять, как работает выбор текста. Узнайте о стилизации и поддержке тёмной темы в руководстве по настройке. Для продвинутых интеграций изучите справочник API.

### Фронтенд-библиотеки

Все фронтенд-библиотеки FastComments (react, vue, angular и т.д.) имеют Collab Chat.