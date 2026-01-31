Далі відкрийте файл `view.php`. Ви можете зробити це за допомогою `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Використайте клавіші стрілок, щоб прокрутити вниз до кінця. Знайдіть текст приблизно такого вигляду:

```php
echo $OUTPUT->box_end();
```

Тепер скопіюємо код, який додає віджет коментарів:

[inline-code-attrs-start title = 'Код коментарів Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Використайте клавіші стрілок, щоб розмістити курсор перед рядком "box_end", і вставте.

У вас має вийти приблизно таке:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Тепер збережіть: 

1. Натисніть `ctrl+x`
2. Натисніть `y`
3. Натисніть `enter`

Готово!