Далі відкрийте файл `view.php`. Ви можете зробити це за допомогою `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Використайте стрілки на клавіатурі, щоб прокрутити донизу. Знайдіть текст, який виглядає приблизно так:

```php
echo $OUTPUT->box_end();
```

Тепер скопіюйте код, який додає віджет коментарів:

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

Використайте стрілки, щоб розмістити курсор перед рядком "box_end" та вставте.

Ви повинні бачити приблизно таке:

<div class="screenshot white-bg">
    <div class="title">Приклад</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Приклад Moodle" />
</div>

Тепер збережіть: 

1. Натисніть `ctrl+x`
2. Натисніть `y`
3. Натисніть `enter`

Ось і все!