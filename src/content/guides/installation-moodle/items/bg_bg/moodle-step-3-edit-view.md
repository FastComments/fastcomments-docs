След това отворете файла `view.php`. Можете да го направите с `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Използвайте стрелките, за да превъртите до дъното. Потърсете текст, който казва нещо като:

```php
echo $OUTPUT->box_end();
```

Сега нека копираме кода, който добавя уиджета за коментари:

[inline-code-attrs-start title = 'Код за коментари в Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Използвайте стрелките, за да позиционирате курсора преди реда "box_end", и поставете.

Трябва да имате нещо подобно на следното:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Сега запазете: 

1. Натиснете `ctrl+x`
2. Натиснете `y`
3. Натиснете `enter`

Готово!