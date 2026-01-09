Далее откройте файл `view.php`. Это можно сделать с помощью `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Используйте клавиши со стрелками, чтобы прокрутить вниз до конца. Найдите строку с текстом примерно такого вида:

```php
echo $OUTPUT->box_end();
```

Теперь скопируем код, который добавляет виджет комментариев:

[inline-code-attrs-start title = 'Код комментариев Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

if ($id) {
    $url_decoded = str_replace('&amp;', '&', $PAGE->url);
    $users_picture_obj = new user_picture($USER);
    $users_picture_url = $users_picture_obj->get_url($PAGE);
    
    $simple_sso_json = json_encode($USER && $USER->username !== 'guest' ? array(
        "username" => $USER->firstname . $USER->lastname,
        "email" => $USER->email,
        "avatar" => $users_picture_url->out(false)
    ) : array(
        "loginURL" => '/login/index.php'
    ));
    
    echo "<script src=\"https://cdn-eu.fastcomments.com/js/embed-v2.min.js\"></script>
    <div id=\"fastcomments-widget\"></div>
    <script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: 'demo',
            simpleSSO: $simple_sso_json,
            urlId: $id,
            url: '$url_decoded'
        });
    </script>";
}
[inline-code-end]

Используйте клавиши со стрелками, чтобы поместить курсор перед строкой "box_end", и вставьте.

У вас должно получиться примерно так:

<div class="screenshot white-bg">
    <div class="title">Пример</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Пример Moodle" />
</div>

Теперь сохраните: 

1. Нажмите `ctrl+x`
2. Нажмите `y`
3. Нажмите `enter`

Готово!