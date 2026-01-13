Vervolgens open je het bestand `view.php`. Je kunt dit doen met `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Gebruik de pijltjestoetsen om naar beneden te scrollen. Zoek naar tekst die er ongeveer zo uitziet:

```php
echo $OUTPUT->box_end();
```

Kopieer nu de code die de commentaarwidget toevoegt:

[inline-code-attrs-start title = 'Moodle Reactiescode'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Gebruik de pijltjestoetsen om je cursor vóór de regel met "box_end" te zetten en plak.

Je zou iets moeten hebben zoals dit:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Sla het nu op: 

1. Druk op `ctrl+x`
2. Druk op `y`
3. Druk op `enter`

Dat is alles!