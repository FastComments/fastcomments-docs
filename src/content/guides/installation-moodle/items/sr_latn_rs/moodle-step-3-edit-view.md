Zatim otvorite fajl `view.php`. Ovo možete uraditi pomoću `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Koristite tastere sa strelicama da skrolujete do dna. Potražite tekst koji izgleda otprilike ovako:

```php
echo $OUTPUT->box_end();
```

Sada ćemo kopirati kod koji dodaje widget za komentare:

[inline-code-attrs-start title = 'Kod komentara za Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Pomerite kursor pomoću tastera sa strelicama pre linije "box_end" i nalepite.

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