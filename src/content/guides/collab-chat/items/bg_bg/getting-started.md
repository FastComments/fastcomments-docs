### Бърз старт

Започването с Collab Chat е просто. Трябват ви скриптът FastComments Collab Chat, HTML елемент, съдържащ текста, който искате да анотирате, и конфигурационен обект с вашия Tenant ID.

### Инсталация

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

Заменете `'demo'` с вашия действителен FastComments Tenant ID, ако вече не сте го направили. Можете да го намерите в [FastComments таблото за управление](https://fastcomments.com/auth/my-account/api-secret).

### Как работи

След инициализация потребителите могат да маркират всеки текст в целевия елемент. След кратко забавяне (3,5 секунди на десктоп) се появява подканващ прозорец, който им позволява да започнат дискусия. Когато се създаде дискусия, върху текста се появява визуално маркиране. Други потребители могат да посочат с мишката или да щракнат върху маркирането, за да видят и се включат в дискусията. Всички дискусии се синхронизират в реално време при всички посетители.

### Демонстрация на живо

Можете да видите Collab Chat в действие на нашата [страница с демонстрация на живо](https://fastcomments.com/product/collab-chat).

### Следващи стъпки

Сега, когато основите работят, можете да персонализирате външния вид и поведението в ръководството Configuration Options. Прегледайте ръководството Text Selection Behavior, за да разберете как работи избирането на текст. Научете повече за стилизирането и поддръжката на тъмен режим в ръководството Customization. За разширени интеграции разгледайте API Reference.

### Frontend библиотеки

Всички FastComments frontend библиотеки (react, vue, angular и др.) имат Collab Chat.