### Брзи почетак

Почетак са Collab Chat-ом је једноставан. Потребни су вам FastComments Collab Chat скрипта, HTML елемент који садржи текст који желите да означите и конфигурациони објекат са вашим Tenant ID.

### Инсталација

Додајте Collab Chat скрипту на вашу страницу:

[inline-code-attrs-start title = 'Учитавање Collab Chat скрипте'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Основна примена

Ево минималног примера:

[inline-code-attrs-start title = 'Основна примена Collab Chat-а'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Замените `'demo'` вашим стварним FastComments Tenant ID-ом ако већ није, који можете пронаћи у вашем [FastComments контролном панелу](https://fastcomments.com/auth/my-account/api-secret).

### Како то функционише

Када се иницијализује, корисници могу изабрати било који текст унутар циљаног елемента. Након кратког кашњења (3.5 секунде на десктопу), појави се подсетник који им омогућава да започну дискусију. Када се дискусија креира, визуелно истакнуће се појављује на тексту. Други корисници могу да задрже курсор изнад или кликну на истакнуће да би видели и учествовали у дискусији. Све дискусије се синхронизују у реалном времену за све посетиоце.

### Демонстрација уживо

Можете видети Collab Chat у акцији на нашој [страници са демонстрацијом уживо](https://fastcomments.com/product/collab-chat).

### Следећи кораци

Сада када имате основну функционалност, можете прилагодити изглед и понашање у водичу Configuration Options. Погледајте водич Text Selection Behavior да бисте разумели како ради селекција текста. Сазнајте о стилизовању и подршци за тамни режим у водичу Customization. За напредне интеграције, истражите API Reference.

### Фронтенд библиотеке

Све FastComments фронтенд библиотеке (react, vue, angular, итд.) садрже Collab Chat.