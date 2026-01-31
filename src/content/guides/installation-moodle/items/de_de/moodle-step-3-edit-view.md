Nächste öffnen Sie die Datei `view.php`. Sie können dies mit `nano` tun:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Verwenden Sie die Pfeiltasten, um nach unten zu scrollen. Suchen Sie nach einem Text, der ungefähr so aussieht:

```php
echo $OUTPUT->box_end();
```

Kopieren Sie nun den Code, der das Kommentar-Widget hinzufügt:

[inline-code-attrs-start title = 'Moodle-Kommentarcode'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Verwenden Sie die Pfeiltasten, um den Cursor vor die Zeile mit "box_end" zu setzen, und fügen Sie ein.

Sie sollten etwa Folgendes haben:

<div class="screenshot white-bg">
    <div class="title">Beispiel</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Beispiel" />
</div>

Jetzt speichern: 

1. Drücken Sie `ctrl+x`
2. Drücken Sie `y`
3. Drücken Sie `enter`

Das war's!