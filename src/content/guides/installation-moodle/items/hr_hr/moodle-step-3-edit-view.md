Zatim otvorite datoteku `view.php`. To možete učiniti pomoću `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Pomoću tipki sa strelicama pomaknite se prema dnu. Potražite tekst koji kaže nešto poput:

```php
echo $OUTPUT->box_end();
```

Sada ćemo kopirati kod koji dodaje widget za komentare:

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

Pomoću tipki sa strelicama postavite kursor prije retka "box_end" i zalijepite.

Trebali biste imati nešto poput ovoga:

<div class="screenshot white-bg">
    <div class="title">Primjer</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle primjer" />
</div>

Sada spremite: 

1. Pritisnite `ctrl+x`
2. Pritisnite `y`
3. Pritisnite `enter`

To je to!