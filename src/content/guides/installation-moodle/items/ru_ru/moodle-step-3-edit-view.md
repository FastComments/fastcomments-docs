Далее откройте файл `view.php`. Вы можете сделать это с помощью `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Используйте стрелки на клавиатуре, чтобы прокрутить вниз до конца. Найдите текст, похожий на:

```php
echo $OUTPUT->box_end();
```

Теперь скопируйте код, который добавляет виджет комментариев:

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
    
    echo "<script async src=\"https://cdn.fastcomments.com/js/embed-v2-async.min.js\"></script>
    <div id=\"fastcomments-widget\"></div>
    <script>
    window.fcConfigs = [{
            target: '#fastcomments-widget',
            tenantId: 'demo',
            simpleSSO: $simple_sso_json,
            urlId: $id,
            url: '$url_decoded'
        }];
    </script>";
}
[inline-code-end]

Используйте стрелки, чтобы поставить курсор перед строкой "box_end" и вставьте.

You should have something like this:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Теперь сохраните: 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

Вот и всё!