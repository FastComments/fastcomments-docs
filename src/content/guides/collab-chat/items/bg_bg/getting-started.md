### Бърз старт

Започването с Collab Chat е просто. Необходим ви е скриптът FastComments Collab Chat, HTML елемент, съдържащ текста, който искате да анотирате, и конфигурационен обект с вашия Tenant ID.

### Инсталиране

Добавете скрипта Collab Chat към вашата страница:

[inline-code-attrs-start title = 'Зареждане на скрипта Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Основна имплементация

Ето минимален пример:

[inline-code-attrs-start title = 'Основна имплементация на Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Контейнер за вашето съдържание -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Заредете скрипта Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Инициализирайте Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Как работи

След като е инициализиран, потребителите могат да изберат произволен текст в целевия елемент. След кратко забавяне (3.5 секунди на десктоп), се появява подсказка, позволяваща им да започнат дискусия. Когато се създаде дискусия, върху текста се появява визуално открояване. Други потребители могат да задържат курсора върху или да кликнат върху открояването, за да видят и участват в дискусията. Всички дискусии се синхронизират в реално време за всички посетители.

### Демо на живо

Можете да видите Collab Chat в действие на нашата [страница с демо на живо](https://fastcomments.com/product/collab-chat).

### Следващи стъпки

Сега когато основите работят, можете да персонализирате външния вид и поведението в ръководството за опции на конфигурацията. Вижте ръководството за поведението при избор на текст, за да разберете как работи селекцията на текст. Научете за стилизиране и поддръжка на тъмен режим в ръководството за персонализиране. За напреднали интеграции разгледайте API справката.

### Фронтенд библиотеки

Всички фронтенд библиотеки на FastComments (react, vue, angular, etc) поддържат Collab Chat.