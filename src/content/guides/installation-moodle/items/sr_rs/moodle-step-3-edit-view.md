---
Затим отворите фајл `view.php`. То можете урадити помоћу `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Користите стрелице на тастатури да скролујете до дна. Потражите текст који изгледа отприлике овако:

```php
echo $OUTPUT->box_end();
```

Сада копирајте код који додаје видгет за коментаре:

[inline-code-attrs-start title = 'Код коментара за Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Користите стрелице на тастатури да поставите курсор пре линије "box_end", и налепите.

Требало би да имате нешто овако:

<div class="screenshot white-bg">
    <div class="title">Пример</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Пример Moodle" />
</div>

Сада сачувајте: 

1. Притисните `ctrl+x`
2. Притисните `y`
3. Притисните `enter`

То је то!

---