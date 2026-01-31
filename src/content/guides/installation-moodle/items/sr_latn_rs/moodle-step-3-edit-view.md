Zatim otvorite datoteku `view.php`. To možete uraditi pomoću `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Koristite tastere sa strelicama da skrolujete do dna. Potražite tekst koji izgleda otprilike ovako:

```php
echo $OUTPUT->box_end();
```

Sada kopirajte kod koji dodaje widget za komentare:

[inline-code-attrs-start title = 'Moodle kod komentara'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Upotrebite tastere sa strelicama da pozicionirate kursor pre linije "box_end", i nalepite.

Treba da imate nešto ovako:

<div class="screenshot white-bg">
    <div class="title">Primer</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle primer" />
</div>

Sada sačuvajte: 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

To je to!