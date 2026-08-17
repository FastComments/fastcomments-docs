### Брзи почетак

Започети са Collab Chat-ом је једноставно. Потребни су вам FastComments Collab Chat скрипт, HTML елемент који садржи текст који желите да анотирате, и објекат конфигурације са вашим Tenant ID-ом.

### Инсталација

Додајте Collab Chat скрипту на вашу страницу:

[inline-code-attrs-start title = 'Učitavanje Collab Chat skripte'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Основна имплементација

Ево минималног примера:

[inline-code-attrs-start title = 'Osnovna implementacija Collab Chat-a'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Ваш контејнер садржаја -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Учитај Collab Chat скрипту -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Иницијализуј Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Замените `'demo'` вашим стварним FastComments Tenant ID-ом ако већ није постављен, који можете пронаћи у вашој [FastComments контролној табли](https://fastcomments.com/auth/my-account/api-secret).

### Како функционише

Након иницијализације, корисници могу да изаберу било који текст унутар циљног елемента. Након кратког кашњења (3,5 секунде на десктопу), појављује се упит који им омогућава да започну дискусију. Када се дискусија креира, визуелно истицање се појављује на тексту. Други корисници могу да пређу мишем преко или кликну на истицање да би видели и учествовали у дискусији. Све дискусије се синхронизују у реалном времену за све посетиоце.

### Живи демо

Можете видети Collab Chat у акцији на нашој [страници живог демоа](https://fastcomments.com/product/collab-chat).

### Следећи кораци

Сада када имате основну функционалност, можете прилагодити изглед и понашање у водичу за **Опције конфигурације**. Погледајте водич за **Понашање при избору текста** да бисте разумели како функционише избор текста. Сазнајте о стилизовању и подршци за тамни режим у водичу за **Прилагођавање**. За напредне интеграције, истражите **API референцу**.

### Фронтенд библиотеке

Све FastComments фронтенд библиотеке (react, vue, angular, итд.) имају Collab Chat.

---