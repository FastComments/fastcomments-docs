Затим отворите датотеку `view.php`. Можете то урадити помоћу `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Користите тастере са стрелицама да скролујете до дна. Потражите неки текст који каже нешто попут:

```php
echo $OUTPUT->box_end();
```

Сада копирајте код који додаје видџет за коментаре:

[inline-code-attrs-start title = 'Moodle код за коментаре'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Помоћу тастера са стрелицама поставите курсор пре линије "box_end" и налепите.

Треба да имате нешто овако:

<div class="screenshot white-bg">
    <div class="title">Пример</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle пример" />
</div>

Сада сачувајте: 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

То је то!