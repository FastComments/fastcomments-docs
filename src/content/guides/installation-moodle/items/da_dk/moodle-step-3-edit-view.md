---
Åbn nu filen `view.php`. Du kan gøre dette med `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Brug piletasterne til at rulle ned til bunden. Find noget tekst, der siger noget i stil med:

```php
echo $OUTPUT->box_end();
```

Kopier nu koden, der tilføjer kommentar-widget'en:

[inline-code-attrs-start title = 'Moodle-kommentarkode'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Brug piletasterne til at placere din markør før linjen "box_end", og indsæt.

Du skulle nu have noget i stil med dette:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Gem nu: 

1. Tryk `ctrl+x`
2. Tryk `y`
3. Tryk `enter`

Det var det!
---